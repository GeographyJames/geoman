# Data Providers Feature — Design Summary

## Overview

A system allowing admins to configure external map data services and expose curated layers from those services to all users on the map view. Data is organised in a three-level hierarchy:

```
Provider      →  Service  →  Layer
DataMap Wales →  WMS      →  SSSI (Wales)
              →  WFS      →  SSSI Boundaries
```

---

## Database Schema

Three tables reflecting the Provider → Service → Layer hierarchy:

The database schema can be seen here: migrations/20260304164248_create_data_providers_services_and_layers_tables.sql

### Why three tables

A provider (e.g. DataMap Wales) can expose multiple service types (WMS, WFS) from the same or different endpoints. Modelled on QGIS's own approach — each service type is a separate connection. The provider table is the natural home for country/subdivision, avoiding duplication across every service row for the same provider.

### Category lives on the layer, not the service

A single service (e.g. Ordnance Survey OS Maps API WMTS) can have both basemap layers (OS Road, OS Outdoor) and overlay layers (OS Greenspace) without any duplication. Because `category` is a column on `data_provider_layers`, the same service row naturally supports both categories — no need for duplicate service entries.

---

## Service Types & Rendering

Each service type maps to an OL source and hook:

| Type             | OL Source                                | Hook                    |
| ---------------- | ---------------------------------------- | ----------------------- |
| `ImageWMS`       | `ImageWMS`                               | `useWmsLayer`           |
| `TileWMS`        | `TileWMS`                                | to be built             |
| `WMTS`           | `WMTS`                                   | to be built             |
| `WFS`            | `VectorSource` + WFS loader              | to be built             |
| `ArcGISRest`     | `VectorSource` + bbox strategy           | `useArcGISFeatureLayer` |
| `MVT`            | `VectorTileSource`                       | `useVectorTileLayer`    |
| `OGCAPIFeatures` | `VectorSource` + OGC API Features loader | to be built             |
| `XYZ`            | `XYZ`                                    | to be built             |

**`ImageWMS` vs `TileWMS`**: Use `ImageWMS` for pay-per-request services (one request per viewport) and `TileWMS` for self-hosted or unlimited services where tile caching (e.g. GeoServer GeoWebCache) gives a performance benefit.

---

## Layer Categories

Each layer belongs to one of two categories (stored on `data_provider_layers.category`):

| Category  | Behaviour                                      | UI Control                                   |
| --------- | ---------------------------------------------- | -------------------------------------------- |
| `overlay` | Multiple active simultaneously, toggled on/off | Checkboxes in layer panel                    |
| `basemap` | Only one active at a time                      | Separate base map selector (design deferred) |

Base maps typically use XYZ tile URLs (OpenStreetMap, Mapbox, Google Maps) or WMTS (Ordnance Survey). Overlay layers are WMS, WFS, or ArcGIS.

---

## WMS and Pay-Per-Request Considerations

- `TileWMS` fires ~10–20 requests per viewport load (one per tile)
- For pay-per-request services, **`ImageWMS` is the safer choice** — one request per viewport rather than per tile
- Server-side caching (GeoServer GWC, Redis) was considered but **likely prohibited by most commercial WMS license terms** — they charge per request precisely to prevent this. Licensing must be reviewed per provider before any caching is implemented.

---

## Admin UI

New **Data Providers** tab in the admin section. Two-column layout.

### Left panel — providers with accordion services

Providers are listed with a DaisyUI accordion. Clicking a provider expands it to reveal its services and simultaneously populates the right panel.

```
┌─────────────────────────────────────────────────┐
│ Region: [All regions ▼]         [+ Add Provider] │  ← region filter
├─────────────────────────────────────────────────┤
│ DataMap Wales (4)  [Wales]  [+ Service] [✏] [🗑] │  ← provider row (collapsed)
├─────────────────────────────────────────────────┤
│ Natural England (3)  [England]  [+ Service] [✏] [🗑] │  ← provider row (expanded)
│   🌐 Natural England WMS  [WMS]  [✏] [🗑]        │  ← service rows
├─────────────────────────────────────────────────┤
│ Scottish Natural Heritage (1)  [Scotland]  ...   │
└─────────────────────────────────────────────────┘
```

- Clicking an expanded provider collapses it and clears the right panel
- Service rows are **informational only** — not clickable. Admins manage services via edit/delete buttons on each row.
- Admin buttons use `stopPropagation` so they don't trigger the accordion
- Layer count shown in brackets next to provider name (total across all categories)

### Right panel — layers table

```
[Overlays (8)]              [Base Maps (4)]          ← category tabs
─────────────────────────────────────────────────
Natural England  [England]  [WMS]
┌──────────────────┬─────────┬──────────────┬──────┬────────┬──┐
│ Name             │ Service │ Identifier   │ Zoom │ Enabled│  │
├──────────────────┼─────────┼──────────────┼──────┼────────┼──┤
│ SSSI England     │ [WMS]   │ NE.SSSI...   │ —    │ ☑      │✏🗑│
│ National Parks   │ [WMS]   │ NE.National… │ z8+  │ ☑      │✏🗑│
│ AONB             │ [WMS]   │ NE.AONB      │z10–16│ ☐      │✏🗑│
└──────────────────┴─────────┴──────────────┴──────┴────────┴──┘
```

- When no provider is selected, the right panel shows **all layers across all providers** with a "Provider" column; clicking a provider name in the table selects it
- When a provider is selected, the Provider column is hidden
- Category tabs filter the layers; counts in the tabs reflect the current scope (all or selected provider)
- The region filter on the left also narrows the right panel when no provider is selected
- The Style column only appears when at least one visible layer has a style configured
- The `enabled` toggle fires a `PATCH` inline — no modal needed
- Edit/Delete via the existing `Modal` + inner form pattern
- **Discover from Capabilities** button on the Add Layer form — fetches `GetCapabilities` (WMS/WMTS) or `FeatureServer?f=json` (ArcGIS) client-side using OL's built-in parsers, presents a checklist of available layers for the admin to select from

---

## Styling

**SLD (Styled Layer Descriptor 1.0) as the universal storage format** for all vector layer styles (WFS, ArcGIS). WMS/WMTS/XYZ styling is server-side so no client style is stored.

Two entry paths, one storage format:

```
Colour picker form  →  buildSld()  →  SLD string  →  style_config JSONB
SLD file upload     →  as-is       →  SLD string  →  style_config JSONB
```

Render path:

```
SLD string → @nieuwlandgeo/sld-reader → OL style function → applied to VectorLayer
```

**Download SLD** — available on any layer that has a style configured. Served as a Blob download client-side (no backend round-trip needed). Enables a full QGIS round-trip:

```
QGIS → export SLD → upload to GeoMan
GeoMan → download SLD → import into QGIS
```

The colour picker uses separate inputs for fill colour, fill opacity, stroke colour, and stroke width — mapping directly to SLD `CssParameter` values. The `style_config` JSONB stores `{ "sld": "<StyledLayerDescriptor>...</StyledLayerDescriptor>" }` regardless of entry path.

---

## Geographic Coverage & Filtering

**ISO standards for region tagging — stored on the provider:**

| Field          | Standard           | Example                      |
| -------------- | ------------------ | ---------------------------- |
| `country_code` | ISO 3166-1 alpha-2 | `GB`, `DE`, `FR`             |
| `subdivision`  | ISO 3166-2         | `GB-SCT`, `GB-WLS`, `GB-ENG` |

Country and subdivision live on `data_providers`, not on services. This avoids repeating the same country on every service row for the same provider. Individual layers can override if needed (e.g. an OS service tagged `GB` with a specific layer tagged `GB-ENG`).

Effective region resolves as:

```
layer.country_code ?? provider.country_code ?? NULL (global)
```

**Admin UI filter** — a region dropdown above the providers list filters by `country_code` / `subdivision`:

```
All regions
Global
United Kingdom (all)
  England
  Scotland
  Wales
  Northern Ireland
```

When no provider is selected, the region filter also narrows the layers shown in the right panel.

**Map layer panel grouping:**

```
Global
United Kingdom
  England
  Scotland
  Wales
  Northern Ireland
```

**Potential future addition Filtering by user**: the existing `operating_country_code` on each user could allow filtering the map layer panel — users only see layers relevant to their operating country by default. The backend could supports this via a query parameter:

```
GET /api/data-provider-layers?country_code=GB
```

A "Show all layers" toggle could bypasses the filter for users who need to cross-reference across regions.

---

## Implementation Order

1. **DB migration** — the three tables with correct foreign keys and constraints
2. **Rust CRUD endpoints** — GET/POST/PATCH/DELETE for all three levels, admin-gated
3. **TanStack Query hooks** — replace mock data with real API calls
4. **Add/Edit/Delete modals** — one set per level, following the existing `Modal` + inner form pattern
5. **Discover from Capabilities** — WMS/WMTS GetCapabilities parser and layer picker UI
6. **Map-side rendering** — wiring enabled overlay layers into `MapLayerControls`, base map selector UI

---

## Implementation Notes (post-build)

### Source schema and layer identifier display

The `source` column is a JSONB blob representing the OL source configuration for a layer. Its shape varies by service type — for example:

| Service type | Expected shape (indicative)                        |
| ------------ | -------------------------------------------------- |
| `ImageWMS`   | `{"layers": "inspire-nrw:NRW_SSSI", "format": "image/png"}` |
| `TileWMS`    | `{"layers": "NE.SSSI", "format": "image/png"}`     |
| `ArcGISRest` | `{"url": "https://...", "layer": 0}`               |
| `XYZ`        | `{"url": "https://{a-c}.tile.openstreetmap.org/{z}/{x}/{y}.png"}` |
| `MVT`        | `{"url": "https://..."}`                           |
| `WFS`        | `{"typeName": "inspire-nrw:NRW_SSSI"}`             |

The original admin UI design included an **Identifier** column in the layers table showing the layer name within the service (e.g. `inspire-nrw:NRW_SSSI`). This was dropped in the initial implementation because the value is embedded inside `source` with no consistent key across service types.

Once the `source` schema is settled per service type, a helper should be added to extract and display the meaningful identifier in the layers table. This makes the table significantly more useful at a glance — the layer name alone doesn't tell you which server-side layer it maps to.

### Forms for structured source fields

The Add/Edit Layer forms currently use a raw JSON textarea for `source`, `style_config`, and `display_options`. This is pragmatic but fragile. Once the `source` schema is settled, the Create/Edit Layer forms should offer structured inputs per service type — e.g. a WMS form with a `layers` text field, `format` dropdown, etc. — rather than requiring the admin to hand-author JSON.

### Service description field removed

The `description` field was removed from `data_provider_services` during initial build — it had no display surface in the UI and added form complexity without value. The column has been dropped from the database. If a description is ever needed at the service level it can be re-added as a migration.

---

## Deferred Decisions

- Base map selector UI design (thumbnail strip, button, etc.) and placement on the map
- Whether the map layer panel hides or collapses irrelevant region sections by default
- Whether disabled layers are hidden or shown greyed-out to non-admin users
- Whether changing a service URL triggers re-discovery of layers
- Rate limiting / cost visibility for pay-per-request WMS services
- Licensing review for any WMS services considered for caching
- Whether to further filter the map layer panel by subdivision (e.g. hide English layers for a Scottish-focused user)
