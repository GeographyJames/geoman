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
| `get_print.rs` / `get_qgis_project.rs` | ❌ commented out — deferred to Steps 5–6 |
| DB `figure/delete` | ❌ commented out |
| `GET /layer-styles` handler + test | ✅ working |
| DB `layer_style` | ✅ uncommented, ported to `Acquire` bound |
| `LayerStyleOutputDTO` | ✅ uncommented |
| `layer_styles` URL | ✅ in `config/urls.yaml` and `urls.rs` |
| DB `project_layer`, `qgis_project` | ❌ commented out |
| `qgis_builder` module | ❌ still commented out — deferred to Step 5 |
| URLs for `project_layers`, `qgis_projects` | ❌ not yet in `config/urls.yaml` |

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

### Step 5 — GET /project-layers

**URLs**
- Add `project_layers: "/project-layers"` to `config/urls.yaml` and `urls.rs`

**DB layer**
- Uncomment `db/project_layer/` and port — likely uses `SelectAllWithParams` with a `ProjectId` param (same pattern as figures list)

**DTOs**
- Uncomment `dtos/project_layer.rs` → `ProjectLayerOutputDTO`

**Handler**
- Port `handlers/project_layer/get.rs` → `get_project_layers`; query param `?project=<id>`
- Register route

**TestApp**
- Add `project_layers_service: HttpService` field

**Tests**
- `get_project_layers_works`

---

### Step 6 — GET /figures/{id}/qgz

**Handler**
- Add `get_figure_qgis_project` to `handlers/figure/get.rs` (or `get_qgis_project.rs`)
- Enable `qgis_builder` module; port `qgis_builder/` imports to geoman paths
- Calls `generate_project(figure, config, &PrintResolution::High, false, PgConfig {...}, None)` — runs fully in-process, no QGIS Server required
- Returns `.qgz` bytes as `application/octet-stream`
- Register the `/qgz` sub-route under `figures_routes`

**Tests**
- `get_figure_qgis_project_works` — asserts response is a valid `.qgz` (zip containing `qgis.qgs`)

---

### Step 7 — GET /figures/{id}/pdf and GET /figures/{id}/jpg

*Requires QGIS Server to be running.*

**DB layer**
- Uncomment `db/qgis_project/` and port `insert`, `select`, `check_unique`

**URLs**
- Add `qgis_projects: "/qgis-projects"` to `config/urls.yaml` and `urls.rs`

**Handler**
- Port `handlers/figure/get_print.rs` → `get_print(FigureId, FigureFormat)`
  - Adaptation: prototype fetches `BaseMapOutputDTO` by ID for the WMS slug; geoman uses `DataProviderLayerId` — needs lookup via `data_provider_layers` to get the layer name/slug for the WMS request
- Port `handlers/qgis_project/get.rs` → `get_qgis_project`
- Register both routes

**TestApp**
- Add `qgis_projects_service: HttpService` field

**Tests**
- `get_figure_pdf_works`, `get_figure_jpg_works`
- Mark `#[ignore]` if QGIS Server is not available in the test environment
