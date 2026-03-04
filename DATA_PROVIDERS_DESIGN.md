# Data Providers Feature — Design Summary

## Overview
A system allowing admins to configure external map data services and expose curated layers from those services to all users on the map view. Data is organised in a three-level hierarchy:

```
Organisation  →  Service  →  Layer
DataMap Wales →  WMS      →  SSSI (Wales)
              →  WFS      →  SSSI Boundaries
```

---

## Database Schema

Three tables reflecting the Organisation → Service → Layer hierarchy:

```sql
CREATE TABLE data_provider_organisations (
  id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  name         TEXT NOT NULL,
  description  TEXT,
  country_code CHAR(2),     -- ISO 3166-1 alpha-2, NULL = global
  subdivision  VARCHAR(10), -- ISO 3166-2, e.g. 'GB-SCT', NULL = whole country
  created_by   TEXT NOT NULL,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE data_provider_services (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  organisation_id UUID NOT NULL REFERENCES data_provider_organisations(id) ON DELETE CASCADE,
  name            TEXT NOT NULL,
  service_type    TEXT NOT NULL CHECK (service_type IN ('WMS', 'WMTS', 'WFS', 'ArcGIS', 'XYZ')),
  base_url        TEXT NOT NULL,
  description     TEXT,
  created_by      TEXT NOT NULL,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE data_provider_layers (
  id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  service_id       UUID NOT NULL REFERENCES data_provider_services(id) ON DELETE CASCADE,
  name             TEXT NOT NULL,
  layer_identifier TEXT NOT NULL,
  category         TEXT NOT NULL DEFAULT 'overlay' CHECK (category IN ('overlay', 'basemap')),
  description      TEXT,
  enabled          BOOLEAN NOT NULL DEFAULT true,
  style_config     JSONB NOT NULL DEFAULT '{}',  -- stores SLD XML string
  display_options  JSONB NOT NULL DEFAULT '{}',  -- opacity, min_zoom, max_zoom
  country_code     CHAR(2),     -- overrides organisation if set
  subdivision      VARCHAR(10), -- overrides organisation if set
  sort_order       INT NOT NULL DEFAULT 0,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Why three tables

An organisation (e.g. DataMap Wales) can expose multiple service types (WMS, WFS) from the same or different endpoints. Modelled on QGIS's own approach — each service type is a separate connection. The organisation table is the natural home for country/subdivision, avoiding duplication across every service row for the same organisation.

### Category lives on the layer, not the service

A single service (e.g. Ordnance Survey OS Maps API WMTS) can have both basemap layers (OS Road, OS Outdoor) and overlay layers (OS Greenspace) without any duplication. Because `category` is a column on `data_provider_layers`, the same service row naturally supports both categories — no need for duplicate service entries.

---

## Service Types & Rendering

Each service type maps to an OL source and hook:

| Type | OL Source | Hook |
|---|---|---|
| WMS | `ImageWMS` | `useWmsLayer` |
| WMTS | `WMTS` | to be built |
| WFS | `VectorSource` + WFS loader | to be built |
| ArcGIS | `VectorSource` + bbox strategy | `useArcGISFeatureLayer` |
| XYZ | `XYZ` | to be built |

---

## Layer Categories

Each layer belongs to one of two categories (stored on `data_provider_layers.category`):

| Category | Behaviour | UI Control |
|---|---|---|
| `overlay` | Multiple active simultaneously, toggled on/off | Checkboxes in layer panel |
| `basemap` | Only one active at a time | Separate base map selector (design deferred) |

Base maps typically use XYZ tile URLs (OpenStreetMap, Mapbox, Google Maps) or WMTS (Ordnance Survey). Overlay layers are WMS, WFS, or ArcGIS.

---

## WMS and Pay-Per-Request Considerations

- `TileWMS` fires ~10–20 requests per viewport load (one per tile)
- For pay-per-request services, **`ImageWMS` is the safer choice** — one request per viewport rather than per tile
- Server-side caching (GeoServer GWC, Redis) was considered but **likely prohibited by most commercial WMS license terms** — they charge per request precisely to prevent this. Licensing must be reviewed per provider before any caching is implemented.

---

## Admin UI

New **Data Providers** tab in the admin section. Two-column layout.

### Left panel — organisations with accordion services

Organisations are listed with a DaisyUI accordion. Clicking an organisation expands it to reveal its services and simultaneously populates the right panel.

```
┌─────────────────────────────────────────────────┐
│ [Overlays]              [Base Maps]              │  ← category tabs
├─────────────────────────────────────────────────┤
│ Region: [All regions ▼]                         │  ← region filter
├─────────────────────────────────────────────────┤
│ DataMap Wales  [Wales]  [+ Service] [✏] [🗑]    │  ← org row (collapsed)
├─────────────────────────────────────────────────┤
│ Natural England  [England]  [+ Service] [✏] [🗑] │  ← org row (expanded)
│   🌐 Natural England WMS  [WMS]  [✏] [🗑]        │  ← service rows
├─────────────────────────────────────────────────┤
│ Scottish Natural Heritage  [Scotland]  ...       │
└─────────────────────────────────────────────────┘
```

- Clicking an expanded org collapses it and clears the right panel
- Service rows are **informational only** — not clickable. Admins manage services via edit/delete buttons on each row.
- Admin buttons use `stopPropagation` so they don't trigger the accordion

### Right panel — all layers for the selected organisation

```
Natural England  [England]  [WMS]          [+ Add Layer]
3 layers across 1 service
┌──────────────────┬─────────┬──────────────┬──────┬────────┬──┐
│ Name             │ Service │ Identifier   │ Zoom │ Enabled│  │
├──────────────────┼─────────┼──────────────┼──────┼────────┼──┤
│ SSSI England     │ [WMS]   │ NE.SSSI...   │ —    │ ☑      │✏🗑│
│ National Parks   │ [WMS]   │ NE.National… │ z8+  │ ☑      │✏🗑│
│ AONB             │ [WMS]   │ NE.AONB      │z10–16│ ☐      │✏🗑│
└──────────────────┴─────────┴──────────────┴──────┴────────┴──┘
```

- Right panel gathers **all layers across all services** of the selected organisation (filtered by current category)
- Each layer row shows a service type badge so the source is clear at a glance
- Right panel header shows org name, region badge, all service type badges, and a layer/service count summary
- The Style column only appears when at least one layer in the org has a style configured
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

**ISO standards for region tagging — stored on the organisation:**

| Field | Standard | Example |
|---|---|---|
| `country_code` | ISO 3166-1 alpha-2 | `GB`, `DE`, `FR` |
| `subdivision` | ISO 3166-2 | `GB-SCT`, `GB-WLS`, `GB-ENG` |

Country and subdivision live on `data_provider_organisations`, not on services. This avoids repeating the same country on every service row for the same organisation. Individual layers can override if needed (e.g. an OS service tagged `GB` with a specific layer tagged `GB-ENG`).

Effective region resolves as:
```
layer.country_code ?? organisation.country_code ?? NULL (global)
```

**Admin UI filter** — a region dropdown above the organisations list filters by `country_code` / `subdivision`:

```
All regions
Global
United Kingdom (all)
  England
  Scotland
  Wales
  Northern Ireland
```

Organisations are only shown if they have at least one service in the current category matching the selected region.

**Map layer panel grouping:**

```
Global
United Kingdom
  England
  Scotland
  Wales
  Northern Ireland
```

**Filtering by user**: the existing `operating_country_code` on each user filters the map layer panel — users only see layers relevant to their operating country by default. The backend supports this via a query parameter:

```
GET /api/data-provider-layers?country_code=GB
```

A "Show all layers" toggle bypasses the filter for users who need to cross-reference across regions.

---

## Implementation Order

1. **DB migration** — the three tables with correct foreign keys and constraints
2. **Rust CRUD endpoints** — GET/POST/PATCH/DELETE for all three levels, admin-gated
3. **TanStack Query hooks** — replace mock data with real API calls
4. **Add/Edit/Delete modals** — one set per level, following the existing `Modal` + inner form pattern
5. **Discover from Capabilities** — WMS/WMTS GetCapabilities parser and layer picker UI
6. **Map-side rendering** — wiring enabled overlay layers into `MapLayerControls`, base map selector UI

---

## Deferred Decisions

- Base map selector UI design (thumbnail strip, button, etc.) and placement on the map
- Whether the map layer panel hides or collapses irrelevant region sections by default
- Whether disabled layers are hidden or shown greyed-out to non-admin users
- Whether changing a service URL triggers re-discovery of layers
- Rate limiting / cost visibility for pay-per-request WMS services
- Licensing review for any WMS services considered for caching
- Whether to further filter the map layer panel by subdivision (e.g. hide English layers for a Scottish-focused user)
