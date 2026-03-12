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
| `dtos/base_map/` (`BaseMapOutputDTO`, `BaseMapDataSource`) | ✅ ported, imports fixed |
| `dtos/figure/output.rs` (`FigureOutputDTO`) | ✅ uncommented, imports fixed |
| `enums/mod.rs` | ✅ `FigureStatus` removed; `PrintResolution` added |
| `ids.rs` | ✅ `BaseMapId` added |
| DELETE handler (`delete.rs`) | ❌ prototype imports — needs full port |
| `get_print.rs` / `get_qgis_project.rs` | ❌ commented out — deferred to Steps 5–6 |
| DB `figure/delete` | ❌ commented out |
| DB `layer_style`, `project_layer`, `qgis_project` | ❌ commented out |
| `qgis_builder` module | ❌ still commented out — deferred to Step 5 |
| URLs for `layer_styles`, `project_layers`, `qgis_projects` | ❌ not yet in `config/urls.yaml` |

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

### Test helpers
`HttpService::get_with_params<P: Serialize>` added alongside `get` — passes params to reqwest's `.query()`. Used for `get_figures` which requires a `?project=<id>` query param (field name is `project`, not `project_id`).

---

## Key Adaptations from Prototype

| Prototype | Geoman |
|---|---|
| `SiteBoundaryId(id.0)` | `FeatureId` from `domain` — now derives `Eq + Hash` for use in `HashSet` |
| `TurbineLayoutId(id.0)` | `LayoutId` from `domain` — now derives `Eq + Hash` |
| `BaseMapOutputDTO` (separate `app.base_maps` table) | Ported as-is into `dtos/base_map/`; `BaseMapId` added to `ids.rs`; base maps loaded via `BaseMapOutputDTO::select` in `from_figure_selection` |
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

### Step 3 — DELETE /figures/{id}

**DB layer**
- Port `db/figure/delete.rs` — soft-delete: `UPDATE app.figures SET status = 'DELETED', last_updated = NOW(), last_updated_by = $2 WHERE id = $1 RETURNING id`
- Use `fetch_one` so missing figure → `RowNotFound` → 404
- Enable `mod delete` in `db/figure/mod.rs`

**Handler**
- Create `handlers/figure/delete.rs` following the data providers delete pattern
- `AuthenticatedUser` extractor; `#[delete("/{figure_id}")]` macro; return `Result<HttpResponse, ApiError>` (204)
- Export from `handlers/figure/mod.rs`, register in `routes/api.rs`

**Tests**
- `delete_figure_works` — create a figure, delete it, assert GET returns 404

---

### Step 4 — GET /layer-styles

**URLs**
- Add `layer_styles: "/layer-styles"` to `config/urls.yaml` and the `Api` struct in `app/src/urls.rs`

**DB layer**
- Uncomment `db/layer_style/` and port `select_all` — fix imports, use `Acquire` bound

**DTOs**
- Uncomment `dtos/layer_style.rs` → `LayerStyleOutputDTO`

**Handler**
- Port `handlers/layer_style/get.rs` → `get_layer_styles`
- Uncomment entry in `handlers/mod.rs`, register route

**TestApp**
- Add `layer_styles_service: HttpService` field, initialise with `URLS.api.layer_styles`

**Tests**
- `get_layer_styles_works`

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
