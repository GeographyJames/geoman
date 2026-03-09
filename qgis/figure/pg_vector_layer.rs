use crate::{
    figure::spec::{QgisLayerSource, QgisLayerSpec, QgisProjectLayer, SupportedEpsg},
    layer::{MapLayer, PgConfig, PgDataSource, PgSource, PgTable, QgisMapLayerBuilder, WkbType},
};

pub fn generate_pg_vector_layer(
    layer: &QgisLayerSpec,
    authcfg: Option<String>,
    pg_config: PgConfig,
) -> Option<MapLayer> {
    if let Some((source, wkb_type, epsg_id)) = match &layer.source {
        QgisLayerSource::ProjectLayer(QgisProjectLayer::Valid {
            table,
            schema,
            wkb_type,
            epsg_id,
        }) => Some((
            PgSource::PgTable(PgTable {
                schema: schema.to_owned(),
                table_name: table.to_owned(),
            }),
            wkb_type.to_owned(),
            (*epsg_id),
        )),

        QgisLayerSource::SiteBoundary { id } => {
            match layer.convert_boundary_to_singleparts {
                false => Some((
                    PgSource::SQL(format!(
                        "SELECT id, name, geom FROM app.project_features WHERE id = {}",
                        id
                    )),
                    WkbType::MultiPolygon,
                    SupportedEpsg::WGS84,
                )),

                true => Some((
                    PgSource::SQL(format!(
                        r"SELECT row_number() over (ORDER BY path) as id, {0} as boundary_id, name, geom
  FROM (
      SELECT pf.name, dump.path, dump.geom
      FROM app.project_features pf, ST_Dump(pf.geom) as dump
      WHERE pf.id = {0}
  ) parts",
                        id
                    )),
                    WkbType::Polygon,
                    SupportedEpsg::WGS84,
                )),
            }
        }
        QgisLayerSource::TurbineLayout { id } => Some((
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
                id
            )),
            WkbType::Point,
            SupportedEpsg::WGS84,
        )),
        QgisLayerSource::ProjectLayer(QgisProjectLayer::Invalid(_)) => None,
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
                legend_text: layer.legend_text.clone(),
                include_on_legend: layer.include_on_legend,
                datasource: crate::layer::DataSource::Postgres(ds),
                srs: Some(epsg_id.into()),
            }
            .build_vector(wkb_type),
        );
    }
    None
}
