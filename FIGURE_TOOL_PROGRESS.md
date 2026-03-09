# Figure Tool — Porting Progress

## Goal

Port the "figure tool" feature from the prototype project into GeoMan. The figure tool allows users to create PDF maps/plans via a form UI. Submitted form data is used to generate a QGIS project file (`.qgz`), which is stored in PostgreSQL and served by QGIS Server to produce PDF or image outputs.

---

## Session 1 (previous session — context compacted)

### Database Migrations

Created and fixed migrations for the four new tables:

- **`public.layer_styles`** — stores QGIS layer style XML (styleqml, stylesld)
- **`app.figures`** — core figure records, referencing `app.projects`, with optional basemap foreign keys into `app.data_provider_layers`, using `app.status` enum (ACTIVE/ARCHIVED/DELETED)
- **`app.figure_layers`** — join table linking figures to their layers; each layer has exactly one source (project_feature_id, turbine_layout_id, or project_layer_source JSONB), enforced by a CHECK constraint
- **`public.qgis_projects`** — stores generated `.qgz` binary content, referencing `app.figures`

Key decisions:

- `qgis_projects` lives in the `public` schema (not a `qgis` schema)
- `site_boundary_id` from the prototype becomes `project_feature_id` referencing `app.project_features`
- `app.status` enum is reused rather than creating a new `figure_status` type
- `qgis_pg_auth_cfg_id` excluded (feature not being ported)

Fixed bugs in `20260304164248_create_data_providers_services_and_layers_tables.sql`:

- Wrong column names in partial unique indexes (`figure_default_main_map_base_map`)
- Wrong table name in one index (`app.data_provider_layers`, not `app.data_provider_base_maps`)

### Architecture Decision: Crate Dependency Resolution

The original prototype was a single crate. Splitting into `domain`, `app`, `qgis` created a circular dependency:

- `domain` referenced `qgis` types (`Extent`, `Size`, `PrintResolution`, `ScalebarUnits`, `CopyrightText`)
- `qgis` referenced `domain` types (`FigureOutputDTO`, `FigureLayerOutputDTO`)

**Resolution:**

- `domain` stays lean — input DTOs and simple enums only, no qgis references
- `qgis` is fully self-contained — defines its own input types (`QgisFigureSpec`, `QgisLayerSpec`, etc.)
- `app` owns the rich output DTOs and is responsible for converting `FigureOutputDTO` → `QgisFigureSpec` before calling `qgis::generate_project()`

Dependency direction: `domain` ← `app` → `qgis` (no cycles)

---

## Session 2 (this morning)

### qgis Crate Refactor — Completed

The `qgis` crate was fully refactored to be self-contained. All 27 existing tests pass.

#### New files

| File                  | Purpose                                                             |
| --------------------- | ------------------------------------------------------------------- |
| `qgis/Cargo.toml`     | Makes qgis a proper workspace crate                                 |
| `qgis/config.rs`      | `QgisFigureConfig` (logo_path, logo_aspect_ratio, north_arrow_path) |
| `qgis/figure/spec.rs` | All qgis-native input types (see below)                             |

#### `qgis/figure/spec.rs` — new input types

- `PrintResolution` — High (300dpi) / Low (96dpi)
- `QgisProjectName(String)` — newtype for project filenames
- `QgisDataProvider` — holds `copyright_text: Option<String>`
- `QgisBasemapSpec` — slug, datasource, data_provider
- `QgisBasemapDataSource` — XYZ / WMS / WMTS variants (named fields)
- `SupportedEpsg` — BNG (27700) / WGS84 (4326)
- `QgisLayerSource` — SiteBoundary { id } / TurbineLayout { id } / ProjectLayer(QgisProjectLayer)
- `QgisProjectLayer` — Valid { schema, table, wkb_type, epsg_id } / Invalid(String)
- `QgisLayerSpec` — name, styleqml, source, legend_text, include_on_legend/map/target, enable_labels, convert_boundary_to_singleparts
- `CopyrightText` — Default / Custom / None
- `QgisFigureProperties` — all figure display properties (title, subtitle, scalebar, north_arrow, etc.)
- `QgisFigureSpec` — the main input to `generate_project()`, with helper methods: `page_size()`, `map_right()`, `layout_name()`, `filename()`, `filename_with_id()`, `qgis_project_name()`, `unique_boundary_ids_on_map()`, `unique_layout_ids_on_map()`, `user_id_with_initials_and_last_updated()`, `map_layer_names()`

#### Files updated

| File                                           | Change                                                                                                                          |
| ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `qgis/mod.rs`                                  | Added `pub mod config;`                                                                                                         |
| `qgis/figure/mod.rs`                           | `generate_project()` now takes `QgisFigureSpec`; `TryFrom` now uses `QgisBasemapDataSource`; removed old `domain`/`app` imports |
| `qgis/figure/figure_builder/mod.rs`            | `FigureBuilder` holds `&QgisFigureSpec` instead of `&FigureOutputDTO`                                                           |
| `qgis/figure/figure_builder/copyright_text.rs` | Uses local `CopyrightText` from spec                                                                                            |
| `qgis/figure/figure_builder/scalebar.rs`       | Fixed self-referential path (`crate::qgis::` → `crate::`)                                                                       |
| `qgis/figure/figure_builder/id.rs`             | Removed `.as_ref()` on plain `i32` fields                                                                                       |
| `qgis/figure/figure_builder/legend.rs`         | Fixed paths; uses external `utils` crate for `format_with_commas`                                                               |
| `qgis/figure/pg_vector_layer.rs`               | Uses `QgisLayerSpec`/`QgisLayerSource`; queries `app.project_features` instead of `app.site_boundaries`                         |
| `qgis/project/mod.rs`                          | `figure_id: i32` (was `Id` newtype); `build_with_layer_styles` takes `Vec<QgisLayerSpec>`                                       |
| `qgis/srs/spatial_ref_system.rs`               | `From<SupportedEpsg>` uses local type from spec                                                                                 |
| `qgis/tests/tests.rs`                          | Removed `domain::dtos::Id` import; replaced `Id(1)` with `1`                                                                    |
| All qgis files                                 | Global replace `crate::qgis::` → `crate::` (self-referential path fix)                                                          |

#### Other fixes

- Added `strum` and `strum_macros` to `qgis/Cargo.toml`
- Added `autotests = false` to `qgis/Cargo.toml` to prevent Cargo from treating `tests/` as integration test binaries (they are inline module tests)
- Added `utils::format_with_commas(u32)` to the `utils` crate
- Added `north_arrow_path` field and `north_arrow_filepath()` method to `QgisFigureConfig`

---

## Remaining Work

### qgis crate

- No further changes needed

### domain crate

- Clean up `domain/src/figure/` — remove qgis references, simplify `FigureOutputDTO`
- Add missing enums: `FigureLayerDatasourceInput`, `FigureLayerDatasourceOutput`, `ProjectLayer`, `ScalebarUnits`
- Fix `domain/src/figure/properties.rs` ScalebarUnits reference
- Update `domain/src/lib.rs` to export figure, figure_layer, layer_style modules

### app crate

- Add `[lib] name = "geoman"` to `app/Cargo.toml`
- Add `qgis = { path = "../qgis" }` to `app/Cargo.toml`
- Create rich `FigureOutputDTO` and conversion to `QgisFigureSpec`
- Add `QgisServerSettings` (with `figure_config: QgisFigureConfig`, `url`) to `AppConfig`
- Add `figures`, `layer_styles`, `qgis_project` to handler modules
- Add URL struct fields and YAML config entries
- Add postgres repo stubs for figures, qgis_projects, layer_styles

### Test infrastructure

Update tests from prototype to work with new project: /home/james/Documents/rust_projects/geoman/app/tests/handlers/api/figures /home/james/Documents/rust_projects/geoman/app/tests/handlers/api/qgis_project and /home/james/Documents/rust_projects/geoman/app/tests/handlers/api/project_layersc
