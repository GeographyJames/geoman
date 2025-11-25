use domain::{GisDataTable, SupportedCrs, TableName, enums::GeometryType};
use ogcapi_types::common::{Bbox, Crs, SpatialExtent};
use sqlx::{PgExecutor, prelude::FromRow};

use crate::repo::{
    RepositoryError,
    traits::{SelectAll, SelectOne},
};

#[derive(FromRow)]
struct GisDataTableRow {
    table_name: String,
    schema_name: String,
    storage_crs_srid: Option<i32>,
    geometry_type: Option<GeometryType>,
    owner: String,
    description: Option<String>,
}

impl GisDataTableRow {
    fn try_into_gis_data_table(
        self,
        table_name: TableName,
        extent: Option<SpatialExtent>,
    ) -> Result<GisDataTable, RepositoryError> {
        let Self {
            schema_name,
            storage_crs_srid,
            geometry_type,

            owner,
            description,
            ..
        } = self;
        let storage_crs = storage_crs_srid.map(Crs::from_srid);
        let supported_crs = SupportedCrs::new(storage_crs.clone());

        Ok(GisDataTable {
            table_name,
            schema_name,
            supported_crs,
            storage_crs,
            geometry_type,
            extent,
            owner,
            description,
        })
    }
}

fn extent_query(table_name: &str) -> String {
    format!(
        r#"
SELECT CASE WHEN bbox IS NOT NULL THEN
                 ARRAY[
                     ST_XMin(bbox),
                     ST_YMin(bbox),
                     ST_XMax(bbox),
                     ST_YMax(bbox)
                 ]
             ELSE NULL
         END FROM (SELECT ST_Extent(ST_Transform(geom, $1))::geometry as bbox FROM gis_data."{table_name}") extent_sub
         "#
    )
}

async fn get_extent<'e, E: ?Sized>(
    executor: &'e E,
    table_name: &str,
    extent_crs: &Crs,
) -> Result<Option<SpatialExtent>, RepositoryError>
where
    &'e E: PgExecutor<'e>,
{
    let extent: Option<Vec<f64>> = sqlx::query_scalar(&extent_query(table_name))
        .bind(extent_crs.as_srid())
        .fetch_one(executor)
        .await?;

    let bbox: Option<Bbox> = extent
        .map(|bbox| Bbox::try_from(bbox.as_slice()))
        .transpose()
        .map_err(|e| {
            RepositoryError::UnexpectedError(anyhow::anyhow!(
                "Layer has invalid bounding box: {}",
                e
            ))
        })?;
    Ok(bbox.map(|bbox| SpatialExtent {
        bbox: vec![bbox],
        crs: extent_crs.clone(),
    }))
}

impl SelectAll for GisDataTable {
    async fn select_all<'e, E>(executor: &'e E) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        &'e E: PgExecutor<'e>,
    {
        let extent_crs = Crs::default();
        let table_rows = sqlx::query_as!(
            GisDataTableRow,
            r#"
SELECT t.tablename as "table_name!",
t.schemaname as "schema_name!",
        g.srid as "storage_crs_srid",
        t.tableowner as "owner!",
        obj_description((t.schemaname || '.' || t.tablename)::regclass) as "description",
        g.type as "geometry_type:GeometryType"
 FROM pg_tables t
INNER JOIN geometry_columns g
ON g.f_table_schema = t.schemaname
AND g.f_table_name = t.tablename
WHERE schemaname = 'gis_data'


        "#
        )
        .fetch_all(executor)
        .await?;
        let mut tables = Vec::new();
        for row in table_rows.into_iter() {
            if let Ok(table_name) = TableName::parse(row.table_name.clone()) {
                let extent = get_extent::<E>(executor, table_name.as_ref(), &extent_crs).await?;

                tables.push(row.try_into_gis_data_table(table_name, extent)?)
            };
        }

        Ok(tables)
    }
}

impl SelectOne for GisDataTable {
    type Id<'a> = TableName;

    async fn select_one<'a, 'e, E>(
        executor: &'e E,
        table_name: Self::Id<'a>,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        &'e E: sqlx::PgExecutor<'e>,
    {
        let extent_crs = Crs::default();
        let row = match sqlx::query_as!(
            GisDataTableRow,
            r#"
SELECT t.tablename AS "table_name!",
t.schemaname as "schema_name!",
        g.srid as "storage_crs_srid",
        t.tableowner as "owner!",
        obj_description((t.schemaname || '.' || t.tablename)::regclass) as "description",
        g.type as "geometry_type:GeometryType"
 FROM pg_tables t
INNER JOIN geometry_columns g
ON g.f_table_schema = t.schemaname
AND g.f_table_name = t.tablename
WHERE tablename = $1
        "#,
            table_name.as_ref()
        )
        .fetch_optional(executor)
        .await?
        {
            Some(row) => row,
            None => return Ok(None),
        };
        let extent = get_extent::<E>(executor, table_name.as_ref(), &extent_crs).await?;

        Some(row.try_into_gis_data_table(table_name, extent)).transpose()
    }
}
