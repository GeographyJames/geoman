# Figure Tool Wiring Plan

## Background

The `figure_tool` feature is being ported from the prototype project (`geodata-manager`) into this workspace. The `qgis` crate has been extracted as a self-contained workspace crate (`qgis/`) with no dependencies on `app` or `domain` — it compiles and all tests pass.

The `figure_tool` feature directory has been copied into `app/src/features/figure_tool/`. Most files exist but have their modules commented out pending wiring. The `post_figure` handler is the first to be fully wired: handler, DB insert, routes, and integration test all pass.

---

## Current State

| Item | Status |
|---|---|
| `qgis` crate | ✅ compiles, tests pass |
| `POST /figures` handler + test | ✅ working |
| `GET /figures` and `GET /figures/{id}` handlers + tests | ✅ working |
| `PATCH /figures/{id}` handler + test | ✅ working |
| `FigureInputDTO`, `FigurePayload` | ✅ present |
| `FigureUpdatePayload` | ✅ added to `handlers/figure/patch.rs` |
| DB `figure/insert`, `figure_layer/insert` | ✅ working |
| DB `figure/select`, `figure_layer/select`, `base_map/select` | ✅ working |
| DB `figure/update` | ✅ working |
| `SelectAll`, `SelectOne`, `SelectAllWithParams` traits | ✅ all use `Acquire` bound |
| `domain::FeatureId`, `domain::LayoutId` | ✅ now derive `Eq + Hash` |
| `dtos/base_map/` (`BaseMapOutputDTO`, `LayerSource`) | ✅ ported; `BaseMapDataSource` replaced by unified `LayerSource` |
| `dtos/figure/output.rs` (`FigureOutputDTO`) | ✅ uncommented, imports fixed |
| `enums/mod.rs` | ✅ `FigureStatus` removed; `PrintResolution` added |
| `ids.rs` | ✅ `BaseMapId` added |
| DELETE handler (`delete.rs`) | ✅ removed — soft delete via PATCH `status: DELETED` |
| `get_print.rs` (renamed to `print.rs`) | ✅ wired up — `GET /figures/{id}/{format}` |
| `get_qgis_project.rs` | ✅ wired up — `GET /qgis-projects/{name}` |
| DB `figure/delete` | ❌ commented out |
| `GET /layer-styles` handler + test | ✅ working |
| DB `layer_style` | ✅ uncommented, ported to `Acquire` bound |
| `LayerStyleOutputDTO` | ✅ uncommented |
| `layer_styles` URL | ✅ in `config/urls.yaml` and `urls.rs` |
| DB `project_layer` | ✅ ported to `SelectAllWithParams` |
| DB `qgis_project` | ✅ ported — `Insert`, `check_unique`, `select_qgis_project` |
| `qgis_builder` module | ✅ enabled — `BaseMapDataSource` replaced by `LayerSource`, orphan `TryFrom` replaced by `build_base_map_layer` free fn |
| URL `qgis_projects` | ✅ in `config/urls.yaml` and `urls.rs` |
| URL `project_layers` | ✅ in `config/urls.yaml` and `urls.rs` |
| `reqwest` features in `app/Cargo.toml` | ✅ `json`, `stream`, `query` |

---

## Porting Approach

### Repository traits
All repo traits (`SelectAll`, `SelectOne`, `SelectAllWithParams`, `Insert`, `Update`) now use the `A: Acquire<'a, Database = Postgres>` bound (owned executor), consistent across the board. This allows impls to call either `.acquire()` for a single connection or `.begin()` for a transaction. The pattern for simple impls:
```rust
let mut conn = executor.acquire().await?;
// use &mut *conn for all queries
```
Update impls that need layer replacement use `conn.begin().await?` for a transaction. The `pg_repo.rs` dispatch methods pass `&self.db_pool` which implements `Acquire`, so call sites are unchanged.

`SelectOneWithParams` and `SelectAllWithParamsStreaming` retain their existing signatures — no changes needed there.

### Figure select approach
`FigureOutputDTO` implements `SelectOne<FigureId>` and `SelectAllWithParams` (with `Params<'a> = ProjectId`) via the standard trait dispatch through `PostgresRepo`. Both acquire a single connection and pass `&mut *conn` through to `from_figure_selection` and then into `select_all_layers_for_figure` and `BaseMapOutputDTO::select`. No transaction is used — sequential queries on one connection.

`FigureSelection` is a private DB-internal struct: no `Serialize`/`Deserialize` derives needed.

Both select queries filter `f.status != 'DELETED'` — list and by-id are consistent.

### Figure update approach (PATCH)
The prototype had a PUT that replaced the entire figure including re-inserting all layers. Geoman uses PATCH instead, matching the rest of the project's conventions.

**`FigureUpdatePayload`** — new struct in `handlers/figure/patch.rs` (same pattern as `TeamUpdatePayload`):
- Non-nullable DB columns (`scale`, `legend_width_mm`, `margin_mm`, `page_width_mm`, `page_height_mm`, `srid`, `status`, `properties`): `Option<T>` — `COALESCE($n, col)` in SQL
- Nullable DB columns (`main_map_base_map_id`, `overview_map_base_map_id`): `Option<Option<DataProviderLayerId>>` with `#[serde(default, deserialize_with = "crate::serde_helpers::double_option")]` — `CASE WHEN $n THEN $v ELSE col END` in SQL
- `layers: Option<Vec<FigureLayerPayload>>` — if `Some`, deletes existing layers and re-inserts; if `None`, leaves layers unchanged
- No `project_id` field — not meaningful for an update

**`qgis_project_uuid` must be regenerated on every PATCH** — the prototype's PUT went through `FigurePayload::into_input_dto()` which always called `uuid::Uuid::new_v4()`. When we ported to `FigureUpdatePayload` + direct SQL, this was initially omitted. The UUID rotation is load-bearing: `qgis_project_name` is derived from it, so a stale UUID means `check_unique` hits the old project and skips the delete-and-reinsert, leaving the low-res project pointing at outdated data. Fixed by adding `qgis_project_uuid = $13` (a fresh `Uuid::new_v4()`) to the UPDATE statement in `db/figure/update.rs`. **If any future PATCH-like operation on figures is added, ensure it also rotates this UUID.**

The DB impl uses a transaction (`conn.begin()`), `RETURNING id` + `fetch_one` so a missing figure returns `RowNotFound` → 404.

**Known limitation:** `properties` uses `COALESCE` which replaces the entire JSONB blob. If the client sends partial `FigureProperties` (e.g. only `title`), all other property fields are overwritten with their `Default` (null) values. A proper fix would use `properties || jsonb_strip_nulls($n::jsonb)` for merge semantics. Deferred — not blocking current tests since figures are freshly created before patching.

### Base map select
Ported as a plain `impl BaseMapOutputDTO { pub async fn select(conn: &mut PgConnection, id: &BaseMapId) }` method, matching the call site in `from_figure_selection`. No trait machinery needed.

### Base map schema migration
The prototype had a dedicated `app.base_maps` / `app.base_map_data_providers` schema. Geoman has no such tables — base maps are `app.data_provider_layers` rows with `category = 'basemap'`, joined through `app.data_provider_services` to `app.data_providers`.

`BaseMapOutputDTO::select` queries `data_provider_layers` directly with aliases to preserve field names:
- `dpl.figure_default_main_map_base_map AS default_main_map_base_map`
- `dpl.figure_default_overview_map_base_map AS default_overview_map_base_map`
- `dpl.source AS datasource`

A `GET /base-maps` handler was briefly added then removed — the existing `GET /data-provider-layers` endpoint already returns all layers and can be filtered by the frontend. `BaseMapOutputDTO` and `BaseMapOutputDTO::select` are kept for use in figure rendering (Steps 6–7).

### LayerSource unification
The prototype's `BaseMapDataSource` (3 variants: WMS, WMTS, XYZ) and the post handler's `LayerSource` (3 different variants: MVT, ArcGISRest, WFS) have been merged into a single canonical `LayerSource` enum in `data_providers/types.rs`.

All 8 variants, tagged with `#[serde(tag = "type", rename_all = "snake_case")]`:

| Variant | Key fields |
|---|---|
| `ImageWms` | `url`, `layers`, `epsg_id`, `alt_project_download_url`, `authcfg_id` |
| `TileWms` | same as `ImageWms` |
| `Wmts` | `url`, `layers`, `tile_matrix_set`, `epsg_id`, `alt_project_download_url`, `authcfg_id` |
| `Xyz` | `url`, `epsg_id`, `authcfg_id` |
| `Mvt` | `url` |
| `ArcGisRest` | `service_name`, `layer_id`, `name_field` |
| `Wfs` | `url`, `layer_name`, `name_field` |
| `OgcApiFeatures` | `url`, `collection_name` |

`epsg_id()` and `set_url_to_alt_url()` methods are implemented on `LayerSource` for use by the qgis builder.

`datasource.rs` in `figure_tool/dtos/base_map/` has been deleted. `BaseMapOutputDTO.datasource` is now `Option<sqlx::types::Json<LayerSource>>`.

**Frontend note:** `EditLayerForm.tsx` duck-types the source variant using field presence. With the tagged enum it will need updating to use `source.type === "mvt"` etc. — deferred.

### DataProviderServiceType sqlx renames
The PostgreSQL enum `app.data_provider_service_type` uses uppercase values (`WMTS`, `WFS`, `MVT`) but the Rust enum uses idiomatic casing (`Wmts`, `Wfs`, `Mvt`). sqlx uses the variant name as-is with no `rename_all`, so three variants need explicit attributes:
```rust
#[sqlx(rename = "WMTS")] Wmts,
#[sqlx(rename = "WFS")]  Wfs,
#[sqlx(rename = "MVT")]  Mvt,
```
The remaining variants (`ImageWMS`, `TileWMS`, `ArcGISRest`, `OGCAPIFeatures`, `XYZ`) match the DB values exactly.

### geoman_migrate base_maps script
`geoman_migrate/src/tables/base_maps.rs` reads `BaseMapDataSource` in the old untagged format from `app.base_maps` and converts to `LayerSource` (tagged) before inserting into `app.data_provider_layers.source`. A local `LayerSource` enum (WMS/WMTS/XYZ variants only) with `#[serde(tag = "type", rename_all = "snake_case")]` is defined in the migration crate — it must stay in sync with the canonical definition in `geoman/app/src/features/data_providers/types.rs`.

### FigureOutputDTO base map fields (deferred)
`FigureOutputDTO` currently has:
```rust
pub main_map_base_map: Option<BaseMapOutputDTO>,
pub overview_map_base_map: Option<BaseMapOutputDTO>,
```
`BaseMapOutputDTO` is a figure-tool-specific projection of `app.data_provider_layers`. Long-term these fields should use `DataProviderLayer` from `data_providers/types.rs` directly, which is the canonical representation of that table. This would also make `BaseMapOutputDTO` and `BaseMapId` redundant and allow them to be removed. Deferred until the qgis_builder is wired up and the full rendering path is tested, since `BaseMapOutputDTO` methods (`set_url_to_alt_url`, `overview_map_slug`) are consumed by `qgis_builder/mod.rs`.

### Base map handling in qgis_builder (deferred — Step 6)
`qgis_builder/mod.rs` contains a `TryFrom<(String, BaseMapDataSource)> for QgisMapLayerBuilder` conversion that maps WMS/WMTS/XYZ datasource fields to the qgis crate's `WMSDataSource`/`XYZDataSource`. This will need porting to use `LayerSource` variants instead when the module is enabled.

Additionally, `BaseMapId` in `figure_tool/ids.rs` is now redundant — it wraps `i32` for the same `app.data_provider_layers.id` column already covered by `DataProviderLayerId`. Similarly `DataProviderId` in `figure_tool/ids.rs` duplicates `data_providers::types::DataProviderId`. These should be consolidated when the qgis_builder is wired up and the full figure rendering path is tested end-to-end.

### Test helpers
`HttpService::get_with_params<P: Serialize>` added alongside `get` — passes params to reqwest's `.query()`. Used for `get_figures` which requires a `?project=<id>` query param (field name is `project`, not `project_id`).

---

## Key Adaptations from Prototype

| Prototype | Geoman |
|---|---|
| `SiteBoundaryId(id.0)` | `FeatureId` from `domain` — now derives `Eq + Hash` for use in `HashSet` |
| `TurbineLayoutId(id.0)` | `LayoutId` from `domain` — now derives `Eq + Hash` |
| `BaseMapOutputDTO` (separate `app.base_maps` table) | Base maps are `app.data_provider_layers` rows with `category = 'basemap'`; `BaseMapOutputDTO::select` queries that table with aliases; `BaseMapDataSource` replaced by unified `LayerSource` from `data_providers/types.rs` |
| `DataProviderServiceType` sqlx renames | ✅ `Wmts`, `Wfs`, `Mvt` variants need explicit `#[sqlx(rename)]` to match uppercase DB enum values |
| `FigureStatus` (prototype-specific enum) | Removed; `domain::enums::Status` used throughout; DB column uses `app.status` (uppercase values: `'ACTIVE'`, `'DELETED'`, etc.) |
| `PrintResolution` (was in `qgis_builder/mod.rs`) | Moved to `enums/mod.rs` so it can be used without enabling `qgis_builder` |
| `app.generate_figure_id(project_id)` | `app.generate_figure_id(auth, project_id)` |
| `app.generate_primary_boundary_id(&project_id)` | `app.generate_primary_boundary_id(project_id, auth)` |
| Session-based auth (`TypedSession`) | `AuthenticatedUser` extractor from `crate::types` |
| `Select` / `SelectAllForProject` prototype traits | `SelectOne<FigureId>` and `SelectAllWithParams` (Params = ProjectId) from geoman's trait system |
| `ProjectId`, `UserId` in figure_tool ids | Come from `domain` — no `AsRef<i32>`, use `.0` directly |
| `app.site_boundaries` | `app.project_features` |
| `fl.user_id` column | Column is `fl.added_by` in geoman — field renamed to `added_by` in `FigureLayerOutputDTO` |
| Prototype `crate::app::` import prefix | `crate::features::`, `crate::config::`, `domain::` directly |
| Handler return type `Result<HttpResponse, actix_web::Error>` | `Result<Json<T>, ApiError>` with `#[get("")]` / `#[get("/{id}")]` macros; PATCH/DELETE return `Result<HttpResponse, ApiError>` (204 No Content) |
| PUT handler (replaces entire figure) | PATCH handler with `FigureUpdatePayload` — partial updates, layers only replaced if provided |

---

## Step-by-Step Plan

### Step 1 — GET /figures and GET /figures/{id} ✅

All passing. See porting approach above for decisions made.

---

### Step 2 — PATCH /figures/{id} ✅

All passing. See figure update approach above.

---

### Step 3 — DELETE /figures/{id} ✅

No separate DELETE handler needed. Soft delete is covered entirely by the existing PATCH endpoint: patch `status: DELETED`, figure is excluded from all queries (both `SelectOne` and `SelectAllWithParams` filter `f.status != 'DELETED'`).

`delete_figure_works` test lives in `tests/features/figure_tool/handlers/figure/patch.rs`:
- Patches `status: Status::Deleted` → asserts 204
- GET by ID → asserts 404
- GET list (`?project=<id>`) → asserts empty vec

The prototype `handlers/figure/delete.rs` and `tests/handlers/figure/delete.rs` have been removed.

---

### Step 4 — GET /layer-styles ✅

All passing. The `layer_styles` table (`public.layer_styles`) is a QGIS-managed table seeded via QGIS Desktop; the test DB starts empty so the test asserts an empty vec rather than checking for content. A POST handler will be added later and can be used to seed test data when needed.

---

### Step 5 — GET /project-layers ✅

All passing.

- URL `project_layers: "/project-layers"` added to `config/urls.yaml` and `urls.rs`
- `db/project_layer/select.rs` ported to `SelectAllWithParams` (`Params = ProjectId`, `MetaData = ()`) — queries `pg_tables` + `geometry_columns` for `project_data` schema, filters by naming regex `^p[0-9]{4}[ _][a-zA-Z0-9_ -]+$`, geometry column `geom`, SRID in `(27700, 4326)`; project filtering done in Rust
- `dtos/project_layer.rs` uncommented — `ProjectLayerOutputDTO` with custom `FromRow` that parses `project_id` and `layer_name` from the table name prefix
- `handlers/project_layer/get.rs` ported — `#[get("")]`, query param `?project=<id>`, returns `Json<Vec<ProjectLayerOutputDTO>>`
- Route registered in `api_routes` via `project_layers_routes`
- `project_layers_service: HttpService` added to `TestApp`
- `execute_sql_file` helper added to `TestApp` — reads and executes a SQL file against the test DB pool using `sqlx::raw_sql`
- Test seeds `seed_data/project_data_tables.sql` (creates `project_data.*` tables including valid and intentionally-invalid variants), asserts 7 valid tables for project 1 and 1 table for project 24

---

### Step 6 — GET /figures/{id}/qgz (merged into Step 7)

**Enable `qgis_builder` module**
- Uncomment `mod qgis_builder` in `features/figure_tool/mod.rs`
- The key adaptation is `TryFrom<(String, BaseMapDataSource)> for QgisMapLayerBuilder` — `BaseMapDataSource` no longer exists; replace with `TryFrom<(String, LayerSource)>`:

| `LayerSource` variant | `DataSource` mapping |
|---|---|
| `Xyz { url, .. }` | `DataSource::XYZ(XYZDataSource { url })` |
| `Wmts { url, layers, tile_matrix_set, epsg_id, authcfg_id, .. }` | `DataSource::WMS(WMSDataSource::new_wmts(authcfg_id, url, layers, epsg_id, tile_matrix_set))` |
| `ImageWms { url, layers, epsg_id, authcfg_id, .. }` | `DataSource::WMS(WMSDataSource::new_wms(authcfg_id, url, layers, epsg_id))` |
| `TileWms { url, layers, epsg_id, authcfg_id, .. }` | same as `ImageWms` |
| MVT / ArcGISRest / WFS / OgcApiFeatures | `Err(anyhow!("unsupported source type for base map: ..."))` |

- `epsg_id()` on `LayerSource` already exists for the CRS/SRS selection logic (27700/4326/3857 → BNG/WGS84/web_mercator)
- `datasource.0` in `generate_project` is now `LayerSource` (was `BaseMapDataSource`) — just update the `TryFrom` target type

**Handler**
- Add `get_figure_qgz` handler (or port from `get_qgis_project.rs` which serves stored `.qgz` — keep that for Step 7; this is a new inline-generate endpoint)
- Calls `generate_project(figure, Some(&config.qgis_server.figure_config), &PrintResolution::High, false, PgConfig { db_name: config.db_settings.database_name, port: config.db_settings.port, host: config.db_settings.host, sslmode: SslMode::from(config.db_settings.require_ssl) }, None)`
- Returns `.qgz` bytes directly as `application/octet-stream` — no DB storage needed for the `/qgz` route
- Register sub-route `/{id}/qgz` under `figures_routes`

**Tests**
- `get_figure_qgis_project_works` — asserts 200 and `content-type: application/octet-stream`; optionally inspect zip bytes

---

### Step 7 — GET /figures/{id}/pdf and GET /figures/{id}/jpg ✅ (code complete, tests require QGIS Server)

*Handler, DB layer, and routes are all wired up. Tests require a running QGIS Server.*

#### What was done
- `db/qgis_project/insert.rs` — `Insert` trait, `Acquire` bound, transaction with delete-before-insert for `low_res = true` rows only; `public.qgis_projects` schema
- `db/qgis_project/check_unique.rs` — method on `QgisProjectName`, `PgExecutor` bound
- `db/qgis_project/select.rs` — free function `select_qgis_project(conn, name)`
- `handlers/figure/print.rs` — handler renamed from `get_print.rs` to avoid module/function name clash; route `#[get("/{figure_id}/{format}")]`; return type `Result<HttpResponse, ApiError>` for `?` operator compatibility
- `handlers/qgis_project/get.rs` — ported and wired to `GET /qgis-projects/{name}`
- `qgis_builder/mod.rs` — `BaseMapDataSource` replaced by `LayerSource`; orphan `TryFrom` replaced by `build_base_map_layer` free fn; `crate::qgis::` → `use qgis::` throughout all sub-modules
- Route: `GET /figures/{figure_id}/{format}` (format = `pdf` or `jpg`)
- Route: `GET /qgis-projects/{name}`
- `app/Cargo.toml` reqwest features: `json`, `stream`, `query`

---

## Post-wiring Review: Issues to Investigate

The following points were flagged after wiring was complete. Work through these before running integration tests.

### ✅ / ❌ Checklist

- [x] **1. High-res project cleanup asymmetry** — intentional. The PDF test explicitly asserts the old high-res project persists after a PATCH (caching behaviour). Low-res (jpg) projects are always regenerated because `qgis_project_uuid` rotates on every PATCH, causing `check_unique` to miss and triggering the delete-and-reinsert.

- [x] **2. `check_unique` / `insert` race condition** — `name` is the primary key on `public.qgis_projects`. Added `ON CONFLICT (name) DO NOTHING` to the INSERT in `db/qgis_project/insert.rs`. `check_unique` is kept as a performance optimisation (skips expensive `generate_project` in the common case); `ON CONFLICT` is the safety net for the rare concurrent-request edge case.

- [x] **3. `GetPrintRequest::default()` hardcoded local db name** — removed the `Default` impl entirely. `GetPrintRequestBuilder::build()` now constructs all fields explicitly, including the previously-constant ones (`service`, `version`, `request`, `crs`, `format`).

- [x] **4. `pg_schema` hardcoded to `"qgis"` in `build_request`** — fixed to `QGIS_PROJECTS_SCHEMA` constant (`"public"`) in `constants.rs`. Tests pass.

- [x] **5. `overview_map_extent` / `map_number` logic** — confirmed correct. `map0` = overview map (when present and `legend_width_mm > 0`), `map1` = main map (or `map0` when no overview). Matches the QGIS layout template's map item numbering.

- [x] **6. CRS for site boundaries and turbine layouts** — since the QGIS project CRS is hardcoded to BNG (27700), all site boundary and turbine layout geometries are now transformed to 27700 in the SQL query via `ST_Transform(geom, 27700)` and the layer CRS declared as `SupportedEpsg::BNG`. This handles source data in any CRS without relying on `SupportedEpsg`'s limited variant set.

- [x] **7. `project.content` encoding** — confirmed via `assert_is_qgis_project` test helper which checks for ZIP magic bytes (`PK\x03\x04`). Content is stored as a raw `.qgz` ZIP blob.

- [x] **8. `get_figure_jpg_works` test gap** — root cause was the missing `qgis_project_uuid` rotation on PATCH (see figure update approach above). Both `get_figure_jpg_works` and `get_figure_pdf_works` now pass. The `qgis_project_uuid` is now rotated on every PATCH via `db/figure/update.rs` (`qgis_project_uuid = uuid::Uuid::new_v4()`). **Any future PATCH-like operation on figures must also rotate this UUID.**

- [x] **9. `FigureFormat` implements `serde::Serialize`** — confirmed: derives `Serialize` and `Deserialize` in `enums/mod.rs`. Tests pass.

- [x] **10. `config.qgis_server.url` trailing slash / path** — no code change needed. `client.get(url).query(&request)` appends query params correctly regardless of trailing slash (no path concatenation). Dev URL is `http://localhost:8001` (root); tests pass. Staging URL is set via `GEOMAN_QGIS_SERVER__URL` env var — operator must provide the full endpoint including any required path.
