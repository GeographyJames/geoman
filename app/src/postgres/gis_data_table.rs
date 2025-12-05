use domain::{GisDataTable, SupportedCrs, TableName, enums::GeometryType};
use ogcapi_types::common::{Bbox, Crs, SpatialExtent};
use sqlx::{FromRow, PgExecutor};

use crate::repo::{
    RepositoryError,
    traits::{SelectAll, SelectOne},
};

#[derive(FromRow)]
struct GisDataTableRow {
    table_name: String,
    schema_name: String,
    storage_crs_srid: Option<i32>,
    geometry_type: Option<String>,
    owner: String,
    description: Option<String>,
}

const QUERY: &str = r#"
SELECT t.tablename as "table_name",
t.schemaname as "schema_name",
        g.srid as "storage_crs_srid",
        t.tableowner as "owner",
        obj_description((t.schemaname || '.' || t.tablename)::regclass) as "description",
        g.type as "geometry_type"
 FROM pg_tables t
INNER JOIN geometry_columns g
ON g.f_table_schema = t.schemaname
AND g.f_table_name = t.tablename

        "#;

fn extent_query(table_name: &TableName) -> String {
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

impl GisDataTableRow {
    fn into_data_table(self, table_name: TableName, extent: Option<SpatialExtent>) -> GisDataTable {
        let Self {
            schema_name,
            storage_crs_srid,
            geometry_type,
            owner,
            description,
            ..
        } = self;
        GisDataTable {
            table_name,
            schema_name,
            storage_crs_srid,
            extent,
            description,
            owner,
            geometry_type,
        }
    }
}

async fn get_extent<'e, E: ?Sized>(
    executor: &'e E,
    table_name: &TableName,
    extent_crs: &Crs,
) -> Option<SpatialExtent>
where
    &'e E: PgExecutor<'e>,
{
    let extent: Option<Vec<f64>> = sqlx::query_scalar(&extent_query(table_name))
        .bind(extent_crs.as_srid())
        .fetch_one(executor)
        .await
        .ok();

    let bbox: Option<Bbox> = extent
        .map(|bbox| Bbox::try_from(bbox.as_slice()))
        .and_then(|r| r.ok());
    bbox.map(|bbox| SpatialExtent {
        bbox: vec![bbox],
        crs: extent_crs.clone(),
    })
}

impl SelectAll for GisDataTable {
    async fn select_all<'e, E>(executor: &'e E) -> Result<Vec<Self>, crate::repo::RepositoryError>
    where
        &'e E: PgExecutor<'e>,
    {
        let extent_crs = Crs::default();
        let mut result = Vec::new();
        let table_rows =
            sqlx::query_as::<_, GisDataTableRow>(&format!("{QUERY} WHERE schemaname = 'gis_data'"))
                .fetch_all(executor)
                .await?;
        for row in table_rows.into_iter() {
            if let Some(table_name) = TableName::parse(row.table_name.to_string()).ok() {
                let extent = get_extent::<E>(executor, &table_name, &extent_crs).await;
                result.push(row.into_data_table(table_name, extent))
            }
        }
        Ok(result)
    }
}

impl SelectOne<TableName> for GisDataTable {
    async fn select_one<'a, E>(
        executor: &'a E,
        table_name: TableName,
    ) -> Result<Option<Self>, RepositoryError>
    where
        Self: Sized,
        &'a E: sqlx::PgExecutor<'a>,
    {
        let extent_crs = Crs::default();

        let row =
            match sqlx::query_as::<_, GisDataTableRow>(&format!("{QUERY} WHERE tablename = $1"))
                .bind(table_name.as_ref())
                .fetch_optional(executor)
                .await?
            {
                Some(row) => row,
                None => return Ok(None),
            };
        let extent = get_extent::<E>(executor, &table_name, &extent_crs).await;
        Ok(Some(row.into_data_table(table_name, extent)))
    }
}
