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

## Session 3

### POST figure — Completed

Handler and test for `POST /api/figures` were ported and passing. See `app/src/handlers/api/figures/post.rs` and `app/tests/handlers/api/figures/post.rs`.

### GET figures — Completed

`GET /api/figures?project_id={id}` and `GET /api/figures/{id}` are implemented and all 3 figure tests pass (117 total pass, 0 failing).

#### What was done

**Domain (`domain/src/`)**

| File | Change |
| ---- | ------ |
| `figure/output.rs` | New `FigureOutputDTO` using typed IDs (`FigureId`, `ProjectId`, `UserId`, `DataProviderLayerId`) and `Status` enum — no qgis dependencies |
| `figure/mod.rs` | Enabled `output` module |
| `figure_layer/figure_layer_datasource.rs` | Re-enabled `ProjectLayer` variant in `FigureLayerDatasourceOutput` (was commented out) |
| `figure_layer/mod.rs` | Exported `FigureLayerDatasourceOutput`, `ProjectLayer`, `SiteAssetDatasourceOutputDTO`, `SiteAssetId` |
| `pg_table/mod.rs` | Exported `WkbType` |
| `pg_table/output.rs` | Added `From<&WkbType> for Geometry` impl |
| `Cargo.toml` (workspace) | Added `serde` feature to `uuid` dependency |

**Postgres (`app/src/postgres/`)**

| File | Change |
| ---- | ------ |
| `figure_layer/select.rs` | Ported from prototype with new typed IDs; `fl.user_id` → `fl.added_by as user_id`; `app.site_boundaries` → `app.project_features`; qualified `fl.properties` to resolve ambiguity with `app.project_features.properties` |
| `figure_layer/mod.rs` | Enabled `select` module |
| `figure/select.rs` | Ported from prototype with new typed IDs; no extent/coord calculation (deferred) |
| `figure/mod.rs` | Enabled `select` module |
| `pg_repo.rs` | Added `get_figures_for_project(ProjectId)` and `get_figure(FigureId)` methods |

**Handlers & Routes**

| File | Change |
| ---- | ------ |
| `handlers/api/figures/get.rs` | New `get_figures` and `get_figure` actix handlers |
| `handlers/api/figures/mod.rs` | Enabled `get` module and exported handlers |
| `routes/api.rs` | Added `get_figures` and `get_figure` to `figure_roots` scope |

**Tests**

| File | Change |
| ---- | ------ |
| `tests/handlers/api/figures/get.rs` | Rewrote to new pattern: `TestApp::with_project()`, explicit auth, `HttpService::get_one`, query param for project filter; complex tests (PDF/JPG/QGZ) commented out pending further porting |
| `tests/handlers/api/figures/mod.rs` | Enabled `get` module |

#### Key differences from prototype

- `FigureOutputDTO` is defined in `domain` (not `app`) and has no `Extent`/`Point`/`map_extent` fields — these are needed for QGIS generation and must be added when porting `get_print`
- Figure layer select: the `site_boundary_id` FK column name is unchanged in the new schema, but it now references `app.project_features` instead of `app.site_boundaries`
- No `BaseMapOutputDTO` fetching — base map data is not joined; only the IDs are returned

---

## Session 4

### PATCH and DELETE figures — Completed

`PATCH /api/figures/{id}` and `DELETE /api/figures/{id}` are implemented and all 5 figure tests pass.

#### What was done

**Postgres (`app/src/postgres/figure/`)**

| File | Change |
| ---- | ------ |
| `figure/update.rs` | Rewrote to implement `Update for (&FigureInputDTO, FigureId, UserId)`; transaction: UPDATE figures row + DELETE existing figure_layers + re-insert new layers |
| `figure/delete.rs` | Rewrote as `impl PostgresRepo { delete_figure() }`; soft-delete via `UPDATE SET status = Status::Deleted` |
| `figure/mod.rs` | Enabled `delete` and `update` modules |

**Handlers & Routes**

| File | Change |
| ---- | ------ |
| `handlers/api/figures/put.rs` | Rewritten as `#[patch("/{id}")]` `patch_figure()`; converts `FigurePayload` → `FigureInputDTO` via `into_input_dto`, then calls `repo.update()` |
| `handlers/api/figures/delete.rs` | Rewritten as `#[delete("/{id}")]` `delete_figure()`; calls `repo.delete_figure()` |
| `handlers/api/figures/mod.rs` | Enabled `put` and `delete` modules; exports `patch_figure`, `delete_figure` |
| `routes/api.rs` | Added `patch_figure` and `delete_figure` to `figure_roots` scope |

**Tests**

| File | Change |
| ---- | ------ |
| `tests/handlers/api/figures/put.rs` | Rewrote to new pattern: `patch_figure_works` — creates figure, PATCHes `scale`, asserts 204 |
| `tests/handlers/api/figures/delete.rs` | Rewrote to new pattern: `delete_figure_works` — creates figure, DELETEs it, asserts 204, verifies it no longer appears in project listing |
| `tests/handlers/api/figures/mod.rs` | Enabled `put` and `delete` modules |

#### Issues identified during this session

Issues 7–10 were identified and added to the Known Issues section below.

---

## Session 5

### get_print test infrastructure and test stubs — Completed

The test infrastructure needed for the `get_print` handler is now in place. Two new tests (`get_figure_pdf_works`, `get_figure_jpg_works`) compile and run; they fail as expected since the handler is not yet ported.

#### What was done

**URL config**

| File | Change |
| ---- | ------ |
| `config/urls.yaml` | Added `qgis_projects: "/qgis-projects"` |
| `app/src/urls.rs` | Added `pub qgis_projects: String` to `Api` struct |

**Test helpers (`app/tests/common/helpers.rs`)**

| Function | Purpose |
| -------- | ------- |
| `assert_response_is_pdf` | Checks `content-type: application/pdf` and `%PDF-` magic bytes |
| `assert_response_is_jpg` | Checks `content-type: image/jpeg` and `\xff\xd8\xff` magic bytes |
| `assert_is_qgis_project` | Checks `content-type: application/octet-stream` and `PK` zip magic bytes |

**Test app (`app/tests/common/test_app.rs`)**

Added `qgis_projects_service: HttpService` pointing at `/api/qgis-projects`.

**Test files**

| File | Change |
| ---- | ------ |
| `tests/handlers/api/figures/get_print/mod.rs` | New module |
| `tests/handlers/api/figures/get_print/get.rs` | `get_figure_pdf_works`, `get_figure_jpg_works` |
| `tests/handlers/api/figures/mod.rs` | Enabled `mod get_print` |

#### Key adaptations from the prototype

- `FigureFormat::pdf`/`::jpg` → hardcoded `"pdf"`/`"jpg"` strings — avoids importing the unported handler type
- `figure.qgis_project_name(&PrintResolution::High)` → `figure.qgis_project_uuid.to_string()` — PDF project name is just the UUID
- `figure.qgis_project_name(&PrintResolution::Low)` → `format!("{}_low-res", figure.qgis_project_uuid)` — derived from `QgisFigureSpec::qgis_project_name` logic
- `VALID_TABLE_NAMES` defined locally — the `project_layers` test module is not yet wired into the test tree
- `put_json` → `patch_json`

#### Also fixed in this session

- `BOUNDARY_CTE` SRID subquery — see Known Issue ~~6~~ (fixed)
- `LAYOUT_CTE` SRID subquery — same fix, scoped to `WHERE layout_id = $1`

---

## Known Issues / Technical Debt

These were identified during the GET porting work and should be addressed before or during the `get_print` port:

### 1. `todo!()` panic in `figure_layer/select.rs` (line 153)
The `else` branch — reached if a `figure_layers` row has no `site_boundary_id`, `turbine_layout_id`, or `project_layer_source` — calls `todo!()`, which panics at runtime. The DB CHECK constraint prevents this state, but the code should return a `RepositoryError` instead of panicking.

### 2. `bounding_box` uses `fetch_one` — wrong error semantics
`query.fetch_one(conn).await?` in the bounding box function maps `sqlx::Error::RowNotFound` to `RepositoryError::RowNotFound`, which the handler returns as a 404. A missing bounding box row is an internal error, not a not-found. Should use `fetch_optional` and return `Ok(None)` for the empty case.

### 3. Status filter is an untyped string literal
`f.status != 'DELETED'` works but bypasses type-safety. Should bind a typed parameter (`.bind(Status::Deleted)`) consistent with the pattern used elsewhere in the codebase.

### 4. `select_figure_with_conn` is dead code
Added in anticipation of QGIS generation needing to fetch a figure within an existing transaction. Remove it until it is actually needed. Currently produces a compiler warning.

### 5. `FigureOutputDTO` is incomplete for QGIS generation
`map_extent`, `target_coord`, `overview_map_extent`, `target_layer_extent` were intentionally omitted. The extent/coord calculation logic from `figure/select.rs` in the prototype will need to be reinstated when porting `get_print`. At that point, `FigureOutputDTO` will either need these fields added, or the conversion to `QgisFigureSpec` will compute them on the fly in the `app` layer.

### 7. `get_figure` does not filter soft-deleted figures
`select_figure` (`GET /api/figures/{id}`) queries `WHERE f.id = $1` with no status filter. After a soft-delete, the endpoint returns the figure with `status: DELETED` rather than 404. Should add `AND f.status != 'DELETED'` (or bind `Status::Deleted`) consistent with `select_figures_for_project`.

### 8. PATCH and DELETE handlers have no ownership or admin guard
Neither `patch_figure` nor `delete_figure` checks that the requesting user owns the figure or has admin rights. Any authenticated user can mutate or soft-delete any figure. Access control should be added once the broader auth model for figures is decided.

### 9. PATCH always generates a new `qgis_project_uuid`
`into_input_dto` calls `uuid::Uuid::new_v4()` unconditionally, so every PATCH invalidates the cached QGIS project file by assigning a new UUID. This is inherited from the prototype and may be intentional (force regeneration on every edit), but should be reviewed when `get_print` is ported — it could cause unnecessary regeneration costs.

### 10. `overvier_map_base_map_id` typo in `FigureInputDTO`
The field `overvier_map_base_map_id` (should be `overview_map_base_map_id`) is a typo carried from the prototype into `domain/src/figure/input.rs`, `payload.rs`, `insert.rs`, and `update.rs`. Harmless but should be corrected before the API stabilises.

### 11. Potential route conflict: `GET /{id}` vs `GET /{id}/{format}`
`get_figure` is registered as `GET /{id}` and `get_print` as `GET /{id}/{format}`. Actix-web should resolve the two-segment path as more specific, but this should be confirmed by the integration tests passing — if actix routes print requests to `get_figure` instead, the result would be a deserialisation error rather than the handler running.

### 12. `get_print` tests require a running QGIS Server
`GET /api/figures/{id}/pdf` and `GET /api/figures/{id}/jpg` both make a real outbound HTTP call to QGIS Server after generating and storing the QGIS project. In the test environment QGIS Server is not running, so the call will fail and the handler returns 500. The tests are most likely failing for this reason. Options: spin up a QGIS Server instance in the test environment, mock the `reqwest::Client`, or restructure the test to assert only that the project was saved (skipping the QGIS Server call). This is the most likely cause of the current 500 errors.

### 13. `database_name` in QGIS project `map=` URI may be wrong in tests
The QGIS project file embeds `postgresql://?dbname={database_name}&schema=public&project={name}`. In tests, `TestApp` creates a UUID-named database per test run; `DatabaseSettings.database_name` will contain that UUID name. QGIS Server would need to be able to connect to that specific database, which is not practical in an automated test. Closely related to issue 12 — both point to the need for a test strategy that does not require QGIS Server.

### 14. `QgisBasemapSpec` import is unused until basemap support is added
In `get_print.rs`, `QgisBasemapSpec` is imported but `basemap` and `overview_basemap` are always `None`. This will produce a compiler warning once the dead-code lint is surfaced. Remove the import or suppress it until basemap fetching is implemented.

### 15. `overview_map_extent` collapses to a point when `legend_width_mm` is 0
Both the width and height of the overview map extent are derived from `legend_width_mm`. If a figure has no legend (`legend_width_mm = 0`), the calculated extent has zero area, which QGIS Server would likely reject. Should guard against zero and fall back to a sensible default size.

### ~~6. SRID subqueries in `BOUNDARY_CTE` and `LAYOUT_CTE` are imprecise~~ — **Fixed**
Both CTEs used unfiltered SRID subqueries that could return a value from the wrong row:
- `BOUNDARY_CTE`: `(SELECT ST_SRID(geom) FROM app.project_features LIMIT 1)` — fixed by adding `WHERE id = $1`, reading the SRID from the specific feature being queried (important because `app.project_features` allows mixed CRS).
- `LAYOUT_CTE`: `(SELECT ST_SRID(geom) FROM app.turbines LIMIT 1)` — fixed by adding `WHERE layout_id = $1 LIMIT 1`, scoping to the correct layout (turbines within a layout share a CRS, but different layouts may differ).

---

---

## Session 6

### get_print handler and qgis_projects — In Progress

All code compiles cleanly. 5/7 figure tests pass. The 2 `get_print` tests (`get_figure_pdf_works`, `get_figure_jpg_works`) fail with 500 — root cause under investigation.

#### Prototype behaviour discovered (important)

The prototype's `Insert` for `QgisProject` wraps in a transaction that **always** deletes existing low-res projects for the same `figure_id` before inserting the new record — regardless of whether the new project is high or low resolution. This is the mechanism that makes the `get_figure_jpg_works` test pass:

- PATCH creates a new `qgis_project_uuid`
- The next JPG request generates `{new_uuid}_low-res`
- The insert deletes all `low_res = true` rows for `figure_id` (including the old `{old_uuid}_low-res`)
- The old name returns 404 ✓

The same `check → generate if absent → insert` flow is used for both PDF and JPG (there is no separate "always regenerate" path for JPG).

#### What was done

**Postgres (`app/src/postgres/`)**

| File | Change |
| ---- | ------ |
| `qgis_projects/mod.rs` | New module: `insert_qgis_project` (transaction: DELETE low-res + INSERT), `qgis_project_exists` (replaces prototype's `check_unique`), `get_qgis_project_content` |
| `mod.rs` | Added `mod qgis_projects;` |
| `pg_repo.rs` | Added `insert_qgis_project`, `qgis_project_exists`, `get_qgis_project_content` methods |

**Handlers & Routes**

| File | Change |
| ---- | ------ |
| `handlers/api/figures/get_print.rs` | Full rewrite — see detail below |
| `handlers/api/figures/mod.rs` | Enabled `pub mod get_print`; re-exports `FigureFormat` |
| `handlers/api/qgis_projects.rs` | New `GET /api/qgis-projects/{name}` handler — returns content bytes or 404 |
| `handlers/api/mod.rs` | Added `pub mod qgis_projects;` |
| `routes/api.rs` | Added `get_print` to `figure_roots`; added `qgis_projects_routes` |
| `startup.rs` | Registered `DatabaseSettings` as `web::Data` so `get_print` can read the DB name |
| `Cargo.toml` | Added `json`, `query`, `stream` features to `reqwest` main dependency |

**`get_print.rs` — key items**

| Item | Notes |
| ---- | ----- |
| `FigureFormat` | `pdf` / `jpg` enum with `Display` |
| `GetPrintRequestBuilder` | Builds QGIS Server WMS GetPrint URL; schema hardcoded to `"public"` (was `"qgis"` in prototype) |
| `get_print` handler | Fetches figure; checks if QGIS project exists; if not, builds `QgisFigureSpec` + calls `generate_project` + saves; then sends GetPrint request to QGIS Server and streams response |
| `build_figure_spec` | Converts `FigureOutputDTO` → `QgisFigureSpec`; `basemap`/`overview_basemap` hardcoded to `None` (TODO) |
| `target_coord` | Centre of target-layer bounding boxes in figure SRID; falls back to UK centre (324636, 673221 BNG) transformed to figure SRID if no target layers |
| `target_layers_bounds` | Unions `bounding_box` of all layers with `include_as_target = true` |
| `default_uk_centre` | Transforms BNG centre to the figure's SRID using GDAL `CoordTransform` |
| `map_extent` | `extent_from_scale(scale, map_width_mm, map_height_mm, target)` — ported from prototype |
| `overview_map_extent` | `extent_from_scale(overview_scale, legend_width_mm, legend_width_mm, target)` — default scale 1,000,000 |
| `convert_layer` | `FigureLayerOutputDTO` → `QgisLayerSpec`; converts datasource variants and `WkbType`/`SupportedEpsg` via string round-trip |
| `convert_properties` | `domain::FigureProperties` → `qgis::QgisFigureProperties`; maps `CopyrightText` and `ScalebarUnits` variants |
| `to_qgis_figure_config` | `app::config::QgisFigureConfig` → `qgis::config::QgisFigureConfig`; computes `logo_aspect_ratio = height / width` |
| `build_qgis_server_request` | Adds `map0:layers`, `map0:extent` (and optionally `map0:GRID_INTERVAL_X/Y`) to the GetPrint request |
| `streaming_response` | Copies response status + headers (excluding `connection`) into an actix `HttpResponseBuilder` |

#### Architectural decisions

- **Extent calculation stays in `app`** — rather than adding `map_extent` etc. to `FigureOutputDTO` (Known Issue #5), the extent is computed on the fly in `get_print.rs` from the layer `bounding_box` values that are already present on `FigureLayerOutputDTO`. This avoids changing the domain DTO.
- **`DatabaseSettings` as app data** — added to `startup.rs` so the handler can supply the DB name to the QGIS project file `map=` URI without carrying the full config.
- **Basemap fetching deferred (TODO)** — `basemap` and `overview_basemap` in `QgisFigureSpec` are always `None` for now. The `DataProviderLayer.source` JSON format would need to be agreed and a fetch-and-convert function added to `build_figure_spec`. Tests don't use basemaps so this does not block the tests.

---

## Remaining Work

### get_print — debug 500 errors

The two `get_print` integration tests return 500. Most likely cause: the handler makes a real outbound HTTP call to QGIS Server after saving the project, and QGIS Server is not running in the test environment (see Known Issues 12 and 13). Confirm with `TEST_LOG=1`, then decide on a test strategy (mock client, dedicated QGIS Server instance, or test only the project-save step).

### Basemap support (TODO)

- Define the JSON format for `DataProviderLayer.source` for basemap layers
- Add a DB fetch + conversion from `DataProviderLayer` → `QgisBasemapSpec` in `build_figure_spec`
- Add corresponding QGIS Server `map0:layers` slug in `build_qgis_server_request`

### Test infrastructure

- Port `tests/handlers/api/qgis_project/` tests (old code, not yet in new pattern)
- Port `tests/handlers/api/project_layers/` tests (directory exists but not wired into test tree)
- Add `get_figure_qgis_project_works` to `get_print/get.rs` once `get_print` tests pass
