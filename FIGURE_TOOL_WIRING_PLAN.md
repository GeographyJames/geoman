# Figure Tool Wiring Plan

## Background

The `figure_tool` feature is being ported from the prototype project (`geodata-manager`) into this workspace. The `qgis` crate has been extracted as a self-contained workspace crate (`qgis/`) with no dependencies on `app` or `domain` — it compiles and all tests pass.

The `figure_tool` feature directory has been copied into `app/src/features/figure_tool/`. Most files exist but have their modules commented out pending wiring. The `post_figure` handler is the first to be fully wired: handler, DB insert, routes, and integration test all pass.

The plan below works through the remaining handlers one at a time, in dependency order.

---

## Current State

| Item | Status |
|---|---|
| `qgis` crate | ✅ compiles, tests pass |
| `POST /figures` handler + test | ✅ working |
| `FigureInputDTO`, `FigurePayload` | ✅ present |
| DB `figure/insert`, `figure_layer/insert` | ✅ working |
| `domain::FeatureId`, `domain::LayoutId` | ✅ now derive `Eq + Hash` |
| `dtos/base_map/` (`BaseMapOutputDTO`, `BaseMapDataSource`) | ✅ ported, imports fixed |
| `dtos/figure/output.rs` (`FigureOutputDTO`) | ✅ uncommented, imports fixed |
| `enums/mod.rs` | ✅ `FigureStatus` removed; `PrintResolution` added |
| `ids.rs` | ✅ `BaseMapId` added |
| All other handlers (`get`, `put`, `delete`, `get_print`) | ❌ commented out |
| DB `figure/select`, `update`, `delete` | ❌ commented out |
| DB `figure_layer/select` | ❌ commented out |
| DB `base_map`, `layer_style`, `project_layer`, `qgis_project` | ❌ commented out |
| `qgis_builder` module | ❌ still commented out — deferred to Step 6 |
| URLs for `layer_styles`, `project_layers`, `qgis_projects` | ❌ not yet in `config/urls.yaml` |

---

## Key Adaptations from Prototype

The geoman project differs from the prototype in a few places that affect the port:

| Prototype | Geoman |
|---|---|
| `SiteBoundaryId(id.0)` | `FeatureId` from `domain` — now derives `Eq + Hash` for use in `HashSet` |
| `TurbineLayoutId(id.0)` | `LayoutId` from `domain` — now derives `Eq + Hash` |
| `BaseMapOutputDTO` (separate `app.base_maps` table) | Ported as-is into `dtos/base_map/`; `BaseMapId` added to `ids.rs`; base map loading from DB deferred to Step 6 (for GET, `main_map_base_map` / `overview_map_base_map` are set to `None`) |
| `FigureStatus` (prototype-specific enum) | Removed; `domain::enums::Status` used throughout; DB column uses `app.status` |
| `PrintResolution` (was in `qgis_builder/mod.rs`) | Moved to `enums/mod.rs` so it can be used without enabling `qgis_builder` |
| `app.generate_figure_id(project_id)` | `app.generate_figure_id(auth, project_id)` |
| `app.generate_primary_boundary_id(&project_id)` | `app.generate_primary_boundary_id(project_id, auth)` |
| Session-based auth (`TypedSession`) | `AuthenticatedUser` extractor |
| `Select` / `SelectAllForProject` traits (prototype repo) | Geoman uses different trait signatures — figure selects are standalone `pub async fn`s |
| `ProjectId`, `UserId` in figure_tool ids | Come from `domain` — no `AsRef<i32>`, use `.0` directly |
| `app.site_boundaries` | `app.project_features` — site boundaries are stored as project features |
| `fl.user_id` column in `app.figure_layers` | Column is `added_by` in geoman — alias as `added_by as user_id` in SELECT |
| Prototype `crate::app::` import prefix | Geoman uses `crate::features::`, `crate::config::`, `domain::` directly |

---

## Step-by-Step Plan

### Step 1 — GET /figures and GET /figures/{id}

**DTOs** ✅
- `dtos/figure/output.rs` uncommented; imports fixed; `FigureStatus` → `Status`; `SiteBoundaryId`/`TurbineLayoutId` → `FeatureId`/`LayoutId`
- `dtos/base_map/` ported; `BaseMapOutputDTO` retained with `#[serde(skip_serializing)]` fields on `FigureOutputDTO` — populated only when needed for qgz/pdf (set to `None` in GET selects)
- `dtos/mod.rs` exports `Point` and `base_map`

**DB layer** ← *next up*
- Port `db/figure/select.rs`:
  - Replace prototype `Select` / `SelectAllForProject` trait impls with two standalone `pub async fn`s: `select_figure(pool, id)` and `select_figures_for_project(pool, project_id)`
  - Set `main_map_base_map = None` and `overview_map_base_map = None` (populated at qgz/pdf time)
  - `project_id.as_ref()` → `project_id.0`
- Port `db/figure_layer/select.rs`:
  - Fix import paths; `SupportedEpsg` from `qgis::srs`; `SiteBoundaryId` → `FeatureId`; `TurbineLayoutId` → `LayoutId`
  - SQL: `fl.user_id` → `fl.added_by as user_id`; JOIN on `fl.added_by`
  - SQL: `app.site_boundaries` → `app.project_features`; update `BOUNDARY_CTE`
- Uncomment both modules in their respective `mod.rs` files

**Handler**
- Add `get_figures` (query param `?project_id=`) and `get_figure` (path `/{id}`) to `handlers/figure/get.rs`
- Uncomment handler exports in `handlers/figure/mod.rs`
- Register routes in `routes/api.rs`

**Tests**
- `get_figures_works`
- `get_figure_works` — adapt boundary layer: `FigureLayerDatasourcePayload::SiteBoundary(boundary.feature_id)`
- `get_figures_works_with_missing_project_layer`

---

### Step 2 — PUT /figures/{id}

**DB layer**
- Port `db/figure/update.rs`

**Handler**
- Port `put_figure` to `handlers/figure/put.rs` (reuses existing `FigurePayload` + `FigureInputDTO`)
- Uncomment export in `handlers/figure/mod.rs`, register route

**Tests**
- `put_figure_works`
  - Note: prototype sends `FigureOutputDTO` as the PUT body; geoman may prefer `FigurePayload` — confirm which the handler accepts

---

### Step 3 — DELETE /figures/{id}

**DB layer**
- Port `db/figure/delete.rs` — soft-delete sets `status = 'DELETED'`
- `FigureStatus` has been removed; `domain::Status` maps to `app.status` enum with `rename_all = "UPPERCASE"` ✅

**Handler**
- Port `delete_figure` to `handlers/figure/delete.rs`
- Register route

**Tests**
- `delete_figure_works`

---

### Step 4 — GET /layer-styles

**URLs**
- Add `layer_styles: "/layer-styles"` to `config/urls.yaml` and the `Api` struct in `app/src/urls.rs`

**DB layer**
- Uncomment `db/layer_style/` and port `select_all`

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
- Uncomment `db/project_layer/` and port `select_all_for_project`

**DTOs**
- Uncomment `dtos/project_layer.rs` → `ProjectLayerOutputDTO`

**Handler**
- Port `handlers/project_layer/get.rs` → `get_project_layers`
- Register route

**TestApp**
- Add `project_layers_service: HttpService` field

**Tests**
- `get_project_layers_works` — uses a numeric `ProjectId` directly

---

### Step 6 — GET /figures/{id}/qgz

**Handler**
- Add `get_figure_qgis_project` to `handlers/figure/get.rs`
- Calls `generate_project(figure, config, &PrintResolution::High, false, PgConfig {...}, None)` — runs fully in-process, no QGIS Server required
- Returns `.qgz` bytes as `application/octet-stream`
- Register the `/qgz` sub-route under `figures_routes` in `routes/api.rs`

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
- Mark `#[ignore]` if QGIS Server is not available in the test environment; otherwise run as integration tests against the live server
