use crate::{
    app::features::figure_tool::{
        dtos::{figure_layer::FigureLayerOutputDTO, pg_table::PgTableOutputDTO},
        enums::{FigureLayerDatasourceOutput, ProjectLayer, SupportedEpsg},
    },
    qgis::layer::{
        MapLayer, PgConfig, PgDataSource, PgSource, PgTable, QgisMapLayerBuilder, WkbType,
    },
};

pub fn generate_pg_vector_layer(
    layer: &FigureLayerOutputDTO,
    authcfg: Option<String>,
    pg_config: PgConfig,
) -> Option<MapLayer> {
    if let Some((source, wkb_type, epsg_id)) = match &layer.source {
        FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Valid(PgTableOutputDTO {
            table,
            schema,
            wkb_type,
            epsg_id,
            ..
        })) => Some((
            PgSource::PgTable(PgTable {
                schema: schema.to_owned(),
                table_name: table.to_owned(),
            }),
            wkb_type.to_owned(),
            (*epsg_id),
        )),

        FigureLayerDatasourceOutput::SiteBoundary(ds) => {
            match layer.properties.convert_boundary_to_singleparts {
                false => Some((
                    PgSource::SQL(format!(
                        "SELECT id, name, geom FROM app.site_boundaries WHERE id = {}",
                        ds.id
                    )),
                    WkbType::MultiPolygon,
                    SupportedEpsg::WGS84,
                )),

                true => Some((
                    PgSource::SQL(format!(
                        r"SELECT row_number() over (ORDER BY path) as id, {0} as boundary_id, name, geom
  FROM (
      SELECT sb.name, dump.path, dump.geom
      FROM app.site_boundaries sb, ST_Dump(sb.geom) as dump
      WHERE sb.id = {0}
  ) parts",
                        ds.id
                    )),
                    WkbType::Polygon,
                    SupportedEpsg::WGS84,
                )),
            }
        }
        FigureLayerDatasourceOutput::TurbineLayout(ds) => Some((
            PgSource::SQL(format!(
                "SELECT t.id,
        l.name as layout_name,
        {0} as layout_id,
        turbine_number,
        hub_height_mm,
        blade_length_mm,
        geom
   FROM app.turbines t
   JOIN app.turbine_layouts l ON l.id = t.layout_id
  WHERE t.layout_id = {0}",
                ds.id
            )),
            WkbType::Point,
            SupportedEpsg::WGS84,
        )),
        FigureLayerDatasourceOutput::ProjectLayer(ProjectLayer::Invalid(_)) => None,
    } {
        let ds = PgDataSource {
            pg_config,
            key: "id".into(),
            srid: Some(epsg_id as u16),
            r#type: Some(wkb_type.clone()),
            checkPrimaryKeyUnicity: 1,
            source,
            geometry_col: "geom".into(),
            authcfg,
        };

        return Some(
            QgisMapLayerBuilder {
                layer_name: layer.name.clone(),
                legend_text: layer.properties.legend_text.clone(),
                include_on_legend: layer.properties.include_on_legend,
                datasource: crate::qgis::layer::DataSource::Postgres(ds),
                srs: Some(epsg_id.into()),
            }
            .build_vector(wkb_type),
        );
    }
    None
}
