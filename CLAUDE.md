# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

GeoMan is a full-stack geospatial data management application. The Rust backend exposes a REST API and an OGC API Features-compliant endpoint, processing shapefiles via GDAL and storing data in PostgreSQL with PostGIS. The React frontend is served as a SPA by the backend.

## Commands

### Backend (Rust)

```bash
# Run with hot reload (excludes frontend and test-tools from watch)
cargo watch -i react-frontend -i test-tools -x run | bunyan

# Run tests
cargo test

# Run a single test by name
cargo test test_name

# Run tests with logging enabled
TEST_LOG=1 cargo test

# Lint and format
cargo clippy
cargo fmt
cargo fmt --check
```

### Frontend (React — run from `react-frontend/`)

```bash
npm run dev        # Vite dev server on port 3000
npm run build      # Production build + tsc type check
npm run test       # Vitest (run once)
npm run tests      # Vitest (watch mode)
npm run lint       # Biome lint
npm run format     # Biome format
npm run check      # Biome lint + format
```

### Database

```bash
# Run migrations
sqlx migrate run

# Prepare sqlx offline query data (run after schema changes)
cargo sqlx prepare --workspace
```

The local development database is PostgreSQL on port `5434` (`postgres://app_local:secret@localhost:5434/geoman_local`). Set `DATABASE_URL` in `.env` to override.

## Architecture

### Workspace Crates

- **`app/`** — Actix-web HTTP server: route registration, request handlers, Postgres repository layer, auth middleware, telemetry
- **`domain/`** — Shared entity types, IDs, DTOs, and enums used across crates
- **`geo/`** — GDAL-based shapefile processing; handles uploads, validation, and EPSG/SRID extraction
- **`ogc/`** — OGC API Features response types and schema definitions
- **`utils/`** — Shared utilities

### Adding a New API Endpoint

1. Define the URL in `config/urls.yaml`
2. Register it in `app/src/urls.rs`
3. Add the route in `app/src/routes/api.rs` (or `ogc_api.rs`)
4. Implement the handler in `app/src/handlers/api/`
5. Add database access in `app/src/postgres/`

### Authentication

- **Production/Staging**: Clerk JWT validation middleware
- **Development**: Mock auth middleware that reads the user from a header (no Clerk required)
- Auth is configured per-route in `app/src/routes/api.rs`
- Test apps use a configurable `AuthService` trait; integration tests use mock auth

### Configuration

Config is loaded from `config/` based on the `GEOMAN_RUN_ENVIRONMENT` env var. Environment variables with the `GEOMAN_` prefix override config file values. Key env vars (see `.env`):

- `DATABASE_URL` — PostgreSQL connection string
- `GEOMAN_AUTH_SETTINGS__CLERK_SECRET_KEY` — Clerk secret
- `TEST_USER_ID`, `TEST_USER_ID_2` — Clerk user IDs for integration tests

### Integration Tests

Tests live in `app/tests/`. The `TestApp` builder in `app/tests/common/test_app.rs` spawns an isolated server instance for each test run, using a unique database (UUID-named) and a random free port. Tests create and tear down their own database state.

### Frontend Data Flow

- API hooks: `src/hooks/api/` — TanStack Query `useQuery`/`useMutation` built on `useApiRequest()` from `src/lib/api.ts`
- Routing: file-based TanStack Router in `src/routes/`
- Errors: backend API errors shown via `addError()` modal; file validation errors shown as inline DaisyUI `alert-warning`

## Important Constraints

- **`utoipa` is locked at v4** — do not upgrade to v5. The OGC test suite is incompatible with utoipa 5.
- **EPSG extraction from `.prj` files**: Esri-style WKT lacks `AUTHORITY` tags, so standard GDAL `SpatialRef` methods fail. The solution is to create a minimal virtual shapefile (dummy `.shp`/`.shx`/`.dbf` + real `.prj`), open it via GDAL, and read the SRID from the layer. Implementation: `geo/src/virtual_shapefile.rs::get_epsg_from_prj()`.
