# Plan: Wire Up figure_tool Feature in geoman

## Context

The figure_tool feature code and tests have been copied from geodata-manager into geoman. The qgis crate is set up as a separate workspace crate. All existing tests pass. The figure_tool module is commented out in `features/mod.rs` — the code still has old import paths and uses old trait signatures from geodata-manager. We need to adapt it to geoman's patterns so it compiles, routes are registered, and the feature is functional.

Tests will be deferred to a follow-up phase — this plan covers source code only.

---

### 1d. Add RepositoryError::Unexpected variant

- **`app/src/repo/error.rs`** — add `#[error("unexpected error: {0}")] Unexpected(anyhow::Error)` variant (figure_tool DB code uses this for JSON serialization and GDAL errors)

### 1e. Add ApiError variants for figure_tool

- **`app/src/errors.rs`** — add `Validation(String)` variant (status 422), handle `RepositoryError::Unexpected` in `From<RepositoryError>` impl

### 1f. Add missing repo traits

- **`app/src/repo/traits.rs`** — add:
  - `Delete<ID>` — `async fn delete(executor, id) -> Result<(), RepositoryError>`
  - `SelectAllForProject<ID>` — using `Acquire` (figure select needs transactions)
  - `CheckUnique` — with associated type `Key`

### 1g. Add PostgresRepo dispatch methods

- **`app/src/postgres/pg_repo.rs`** — add `delete`, `select_all_for_project`, `check_unique` methods

**Verify:** `cargo check` passes (figure_tool not yet compiled)

---

## Step 2: Fix leaf modules — ids, enums, entities, DTOs

Fix imports in all data-only modules. The universal change is:

- `crate::app::features::figure_tool::` → `crate::features::figure_tool::`
- `crate::app::configuration::QgisFigureConfig` → `crate::config::QgisFigureConfig`
- `crate::domain::dtos::UserId` → `domain::UserId`
- `crate::qgis::` → `qgis::`

**Files** (only those with imports to fix):

- `enums/mod.rs` — fix `crate::app::` and `crate::qgis::srs::SupportedEpsg`
- `dtos/figure/properties.rs` — fix `crate::app::` and `crate::qgis::enums::`
- `dtos/figure/input.rs` — fix `crate::app::`, `crate::domain::dtos::UserId`, config import
- `dtos/figure/output.rs` — fix `crate::app::`, `crate::domain::dtos::UserId`, `crate::qgis::`
- `dtos/figure_layer/input.rs` — fix `crate::app::`
- `dtos/figure_layer/output.rs` — fix `crate::app::`, `crate::domain::dtos::UserId`
- `dtos/base_map/output.rs` — fix `crate::app::`
- `dtos/layer_style.rs` — fix `crate::app::`
- `dtos/project_layer.rs` — fix `crate::app::`, `crate::qgis::layer::`
- `dtos/pg_table/output.rs` — fix `crate::app::`, `crate::qgis::layer::`

**No changes needed:** `ids.rs`, `entities.rs`, `dtos/bounding_box.rs`, `dtos/figure_layer/properties.rs`, `dtos/base_map/datasource.rs`, `dtos/pg_table/input.rs`

---

## Step 3: Fix qgis_builder module + remove authcfg

### 3a. Remove authcfg parameter chain (per FIGURE_TOOL_PORT.md)

- `qgis_builder/mod.rs` — remove `authcfg: Option<String>` from `generate_project()` signature and forwarding call
- `qgis_builder/pg_vector_layer.rs` — remove `authcfg: Option<String>` param, set `PgDataSource.authcfg` to `None`

### 3b. Fix imports in all qgis_builder files

- `qgis_builder/mod.rs` — fix config, dtos, qgis imports
- `qgis_builder/pg_vector_layer.rs` — fix `crate::app::`, `crate::qgis::`
- `qgis_builder/figure_builder/mod.rs` — fix config, dtos, qgis imports
- `qgis_builder/figure_builder/legend.rs` — fix `crate::qgis::` → `qgis::`, replace `utils::format_with_commas` with local helper (inline the 12-line function)
- `qgis_builder/figure_builder/copyright_text.rs` — fix `crate::qgis::`, `crate::app::`
- `qgis_builder/figure_builder/scalebar.rs` — fix `crate::qgis::`
- `qgis_builder/figure_builder/north_arrow.rs` — fix `crate::qgis::`
- `qgis_builder/figure_builder/text_box.rs` — fix `crate::qgis::`
- `qgis_builder/figure_builder/id.rs` — fix `crate::qgis::`

---

## Step 4: Adapt DB implementations to geoman's repo traits

The most complex step. Key changes:

- Old `Insert<&PgPool, FigureId>` → new `Insert { type Id = FigureId; }` with `Acquire` executor
- Old `Update<&PgPool, FigureId>` takes separate id → new `Update for (FigureInputDTO, FigureId)`
- Old `Select` returns `Result<Self, _>` → new `SelectOne` returns `Result<Option<Self>, _>`
- Old `SelectAll<REPO>` → new `SelectAll` with `PgExecutor`
- `RepositoryError::UnexpectedError(...)` → `RepositoryError::Unexpected(...)`

**Files:**

| File                              | Old trait                                                                        | New trait                                                 |
| --------------------------------- | -------------------------------------------------------------------------------- | --------------------------------------------------------- |
| `db/figure/insert.rs`             | `Insert<&PgPool, FigureId>`                                                      | `Insert { type Id = FigureId }`                           |
| `db/figure/update.rs`             | `Update<&PgPool, FigureId>`                                                      | `Update for (FigureInputDTO, FigureId)`                   |
| `db/figure/select.rs`             | `SelectAllForProject<&PgPool, ProjectId>`, `Select<&mut PgConnection, FigureId>` | `SelectAllForProject<&ProjectId>`, `SelectOne<&FigureId>` |
| `db/figure/delete.rs`             | `Delete<REPO, ID>`                                                               | `Delete<&FigureId>`                                       |
| `db/base_map/select.rs`           | `SelectAll<REPO>`, `Select<&mut PgConnection, BaseMapId>`                        | `SelectAll`, `SelectOne<&BaseMapId>`                      |
| `db/layer_style/select.rs`        | `SelectAll<&PgPool>`                                                             | `SelectAll`                                               |
| `db/project_layer/select.rs`      | `SelectAllForProject<&PgPool, ProjectId>`                                        | `SelectAllForProject<&ProjectId>`                         |
| `db/qgis_project/insert.rs`       | `Insert<&PgPool, String>`                                                        | `Insert { type Id = String }`                             |
| `db/qgis_project/select.rs`       | `Select<&mut PgConnection, QgisProjectName>`                                     | `SelectOne<&QgisProjectName>`                             |
| `db/qgis_project/check_unique.rs` | `CheckUnique<REPO, QgisProjectName>`                                             | `CheckUnique { type Key = QgisProjectName }`              |
| `db/figure_layer/insert.rs`       | helper fn (not trait)                                                            | import fixes only                                         |
| `db/figure_layer/select.rs`       | helper fn (not trait)                                                            | import fixes only                                         |

**Key complication — `db/figure/select.rs`:** Both `SelectAllForProject` and `SelectOne` impls need transactions internally (sub-queries for layers, base maps). The `SelectAllForProject` trait uses `Acquire` which supports `begin()`. For `SelectOne`, since `PgExecutor` doesn't have `begin()`, either:

- Use `Acquire` in the `SelectOne` trait for this type specifically, or
- Put figure-specific select logic as a direct method on `PostgresRepo`

**Recommendation:** Keep `SelectOne<&FigureId>` for `FigureOutputDTO` but internally call `from_figure_selection` using the pool (which implements `Acquire`). Alternatively, add a `select_figure` method to `PostgresRepo` directly. We'll decide during implementation based on what compiles cleanly.

**All files:** also fix `crate::app::` → `crate::` import paths.

---

## Step 5: Adapt handlers

Common changes across ALL handlers:

- `TypedSession` + `user_id(&session)?` → `web::ReqData<AuthenticatedUser>` + `user.id`
- `web::Data<Settings>` → `web::Data<QgisServerSettings>` + `web::Data<DatabaseSettings>` where needed
- `ApiError::Repository { source, message }` → just `?` (auto-convert)
- `config.qgis_server.figure_config` → `config.figure_config`
- `config.database.*` → `db_config.*`
- Return type → `Result<HttpResponse, ApiError>` or `Result<Json<T>, ApiError>`
- `repo.select(...)` → `repo.select_one::<T, _>(...).await?.ok_or(ApiError::NotFound)?`
- `repo.update(&dto, &id)` → `repo.update(&(dto, id))`

**Files:**

- `handlers/figure/payload.rs` — import fixes
- `handlers/figure/post.rs` — auth, config, error changes
- `handlers/figure/get.rs` — auth, error, `select_one` changes, **remove `UserOutputDTO` + authcfg from `get_figure_qgis_project`**
- `handlers/figure/put.rs` — auth, config, error, update signature
- `handlers/figure/delete.rs` — error changes
- `handlers/figure/get_print.rs` — config split, `streaming_response` helper, error, select changes
- `handlers/base_map/get.rs` — error changes
- `handlers/layer_style/get.rs` — error changes
- `handlers/project_layer/get.rs` — error changes
- `handlers/qgis_project/get.rs` — error, `select_one` changes

**New file:** `handlers/helpers.rs` — `streaming_response()` helper (15-line function, copy from geodata-manager)

---

## Step 6: Uncomment module + fix compilation

- **`app/src/features/mod.rs`** — uncomment `pub mod figure_tool;`
- Run `cargo check`, fix remaining errors iteratively

---

## Step 7: URL + route registration

- **`config/urls.yaml`** — add `figures`, `base_maps`, `project_layers`, `layer_styles`, `qgis_projects` under `api:`
- **`app/src/urls.rs`** — add corresponding fields to `Api` struct
- **`app/src/routes/api.rs`** — import handlers, add `figure_routes`, `base_map_routes`, `project_layer_routes`, `layer_style_routes`, `qgis_project_routes` functions, wire into `api_routes`

---

## Step 8: Build verification + cleanup

- `cargo build` — full build
- `cargo clippy` — fix warnings
- Verify handler visibility and re-exports

---

## Verification

1. `cargo check` after each step
2. `cargo build` at the end
3. `cargo test --no-run` to confirm tests compile (they won't be wired up yet but shouldn't break existing tests)
