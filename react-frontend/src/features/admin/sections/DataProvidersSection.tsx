import { useState } from "react";
import { Globe, Plus, Pencil, Trash2, Layers, MapPin, Map } from "lucide-react";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";

type ServiceType = "ImageWMS" | "TileWMS" | "WMTS" | "WFS" | "ArcGISRest" | "MVT" | "OGCAPIFeatures" | "XYZ";
type ServiceCategory = "overlay" | "basemap";

interface StyleConfig {
  fillColor: string;
  strokeColor: string;
}

interface DataProvider {
  id: string;
  name: string;
  description?: string;
  country_code: string | null;
  subdivision: string | null;
}

interface DataService {
  id: string;
  provider_id: string;
  name: string;
  service_type: ServiceType;
  base_url: string;
}

interface DataProviderLayer {
  id: string;
  service_id: string;
  name: string;
  layer_identifier: string;
  category: ServiceCategory;
  enabled: boolean;
  description?: string;
  min_zoom?: number;
  max_zoom?: number;
  style_config?: StyleConfig;
}

type LayerRow = DataProviderLayer & { service: DataService; provider: DataProvider };

const MOCK_PROVIDERS: DataProvider[] = [
  { id: "p1", name: "DataMap Wales",            country_code: "GB", subdivision: "GB-WLS" },
  { id: "p2", name: "Natural England",           country_code: "GB", subdivision: "GB-ENG" },
  { id: "p3", name: "Scottish Natural Heritage", country_code: "GB", subdivision: "GB-SCT" },
  { id: "p4", name: "Ordnance Survey",           country_code: "GB", subdivision: null },
  { id: "p5", name: "OpenStreetMap",             country_code: null, subdivision: null },
  { id: "p6", name: "Mapbox",                    country_code: null, subdivision: null },
];

const MOCK_SERVICES: DataService[] = [
  { id: "svc1", provider_id: "p1", name: "DataMap Wales WMS",       service_type: "ImageWMS",      base_url: "https://datamap.gov.wales/geoserver/ows" },
  { id: "svc2", provider_id: "p1", name: "DataMap Wales WFS",       service_type: "WFS",           base_url: "https://datamap.gov.wales/geoserver/ows" },
  { id: "svc3", provider_id: "p2", name: "Natural England WMS",     service_type: "ImageWMS",      base_url: "https://environment.data.gov.uk/spatialdata/sites-of-special-scientific-interest-sssi/wms" },
  { id: "svc4", provider_id: "p3", name: "Scottish SNH ArcGIS",     service_type: "ArcGISRest",    base_url: "https://services1.arcgis.com/LM9GyVFsughzHdbO/ArcGIS/rest/services" },
  { id: "svc5", provider_id: "p4", name: "OS Maps API",             service_type: "WMTS",          base_url: "https://api.os.uk/maps/raster/v1/wmts" },
  { id: "svc6", provider_id: "p5", name: "OpenStreetMap",           service_type: "XYZ",           base_url: "https://{a-c}.tile.openstreetmap.org/{z}/{x}/{y}.png" },
  { id: "svc7", provider_id: "p6", name: "Mapbox Streets",          service_type: "XYZ",           base_url: "https://api.mapbox.com/styles/v1/mapbox/streets-v12/tiles/{z}/{x}/{y}" },
  { id: "svc8", provider_id: "p6", name: "Mapbox Satellite",        service_type: "XYZ",           base_url: "https://api.mapbox.com/styles/v1/mapbox/satellite-v9/tiles/{z}/{x}/{y}" },
  { id: "svc9", provider_id: "p6", name: "Mapbox Vector Tiles",     service_type: "MVT",           base_url: "https://api.mapbox.com/v4/mapbox.mapbox-streets-v8/{z}/{x}/{y}.mvt" },
];

const MOCK_LAYERS: Record<string, DataProviderLayer[]> = {
  "svc1": [
    { id: "l1",  service_id: "svc1", name: "SSSI (Wales)",     layer_identifier: "inspire-nrw:NRW_SSSI", category: "overlay", enabled: true,  description: "Sites of Special Scientific Interest" },
    { id: "l2",  service_id: "svc1", name: "Ancient Woodland", layer_identifier: "inspire-nrw:NRW_AW",   category: "overlay", enabled: true  },
    { id: "l3",  service_id: "svc1", name: "Flood Risk Areas", layer_identifier: "nrw:flood_risk",       category: "overlay", enabled: false, description: "Flood risk zone boundaries", min_zoom: 12 },
  ],
  "svc2": [
    { id: "l4",  service_id: "svc2", name: "SSSI Boundaries",  layer_identifier: "inspire-nrw:NRW_SSSI", category: "overlay", enabled: true, style_config: { fillColor: "rgba(34, 197, 94, 0.2)", strokeColor: "#16A34A" } },
  ],
  "svc3": [
    { id: "l5",  service_id: "svc3", name: "SSSI England",                        layer_identifier: "NE.SitesOfSpecialScientificInterest", category: "overlay", enabled: true  },
    { id: "l6",  service_id: "svc3", name: "National Parks",                      layer_identifier: "NE.NationalParks",                    category: "overlay", enabled: true,  min_zoom: 8 },
    { id: "l7",  service_id: "svc3", name: "Areas of Outstanding Natural Beauty", layer_identifier: "NE.AONB",                             category: "overlay", enabled: false, min_zoom: 10, max_zoom: 16 },
  ],
  "svc4": [
    { id: "l8",  service_id: "svc4", name: "Scottish SSSI", layer_identifier: "/0", category: "overlay", enabled: true, min_zoom: 12, style_config: { fillColor: "rgba(220, 38, 38, 0.15)", strokeColor: "#DC2626" } },
  ],
  "svc5": [
    { id: "l9",  service_id: "svc5", name: "OS Road",           layer_identifier: "Road_3857",          category: "basemap", enabled: true  },
    { id: "l10", service_id: "svc5", name: "OS Outdoor",        layer_identifier: "Outdoor_3857",       category: "basemap", enabled: true  },
    { id: "l11", service_id: "svc5", name: "OS Light",          layer_identifier: "Light_3857",         category: "basemap", enabled: true  },
    { id: "l12", service_id: "svc5", name: "OS Greenspace",     layer_identifier: "Greenspace_3857",    category: "overlay", enabled: true  },
    { id: "l13", service_id: "svc5", name: "OS Open Zoomstack", layer_identifier: "OpenZoomstack_3857", category: "overlay", enabled: false },
  ],
  "svc6": [
    { id: "l14", service_id: "svc6", name: "OpenStreetMap Standard", layer_identifier: "{z}/{x}/{y}.png", category: "basemap", enabled: true },
  ],
  "svc7": [
    { id: "l15", service_id: "svc7", name: "Streets",   layer_identifier: "streets-v12",  category: "basemap", enabled: true },
  ],
  "svc8": [
    { id: "l16", service_id: "svc8", name: "Satellite", layer_identifier: "satellite-v9", category: "basemap", enabled: true },
  ],
};

const SERVICE_TYPE_BADGE: Record<ServiceType, string> = {
  ImageWMS:      "badge-info",
  TileWMS:       "badge-info",
  WMTS:          "badge-warning",
  WFS:           "badge-success",
  ArcGISRest:    "badge-secondary",
  MVT:           "badge-primary",
  OGCAPIFeatures:"badge-success",
  XYZ:           "badge-accent",
};

const SUBDIVISION_LABELS: Record<string, string> = {
  "GB-ENG": "England",
  "GB-SCT": "Scotland",
  "GB-WLS": "Wales",
  "GB-NIR": "N. Ireland",
};

const COUNTRY_LABELS: Record<string, string> = {
  GB: "United Kingdom",
  DE: "Germany",
  FR: "France",
  US: "United States",
};

type RegionFilter = "" | "global" | string;

function matchesFilter(provider: DataProvider, filter: RegionFilter): boolean {
  if (!filter) return true;
  if (filter === "global") return provider.country_code === null;
  if (filter.includes("-")) return provider.subdivision === filter;
  return provider.country_code === filter && provider.subdivision === null;
}

function RegionBadge({ countryCode, subdivision }: { countryCode: string | null; subdivision: string | null }) {
  if (!countryCode) return <span className="badge badge-ghost badge-sm text-xs">Global</span>;
  if (subdivision) return <span className="badge badge-ghost badge-sm text-xs">{SUBDIVISION_LABELS[subdivision] ?? subdivision}</span>;
  return <span className="badge badge-ghost badge-sm text-xs">{COUNTRY_LABELS[countryCode] ?? countryCode}</span>;
}

function ZoomBadge({ minZoom, maxZoom }: { minZoom?: number; maxZoom?: number }) {
  if (minZoom == null && maxZoom == null) return <span className="text-base-content/30">—</span>;
  const label = minZoom != null && maxZoom != null
    ? `z${minZoom}–${maxZoom}`
    : minZoom != null ? `z${minZoom}+` : `–z${maxZoom}`;
  return <span className="badge badge-ghost badge-sm font-mono text-xs">{label}</span>;
}

function ColourSwatch({ styleConfig }: { styleConfig?: StyleConfig }) {
  if (!styleConfig) return <span className="text-base-content/30">—</span>;
  return (
    <svg width="24" height="16" className="shrink-0">
      <rect x="1" y="1" width="22" height="14" rx="2" fill={styleConfig.fillColor} stroke={styleConfig.strokeColor} strokeWidth="1.5" />
    </svg>
  );
}

export default function DataProvidersSection() {
  const { data: currentUser } = useCurrentUser();
  const [selectedProvider, setSelectedProvider] = useState<DataProvider | null>(null);
  const [layers, setLayers] = useState(MOCK_LAYERS);
  const [regionFilter, setRegionFilter] = useState<RegionFilter>("");
  const [category, setCategory] = useState<ServiceCategory>("overlay");

  const isAdmin = currentUser?.isAdmin ?? false;

  const filteredProviders = MOCK_PROVIDERS.filter((provider) => {
    if (!matchesFilter(provider, regionFilter)) return false;
    return MOCK_SERVICES.some((s) => s.provider_id === provider.id);
  });

  const providerServices = selectedProvider
    ? MOCK_SERVICES.filter((s) => s.provider_id === selectedProvider.id)
    : [];

  const activeServices = selectedProvider
    ? providerServices
    : MOCK_SERVICES.filter((s) => filteredProviders.some((p) => p.id === s.provider_id));

  const displayLayers: LayerRow[] = activeServices.flatMap((service) => {
    const provider = MOCK_PROVIDERS.find((p) => p.id === service.provider_id)!;
    return (layers[service.id] ?? [])
      .filter((l) => l.category === category)
      .map((l) => ({ ...l, service, provider }));
  });

  const overlayCount = activeServices.reduce((n, s) => n + (layers[s.id] ?? []).filter((l) => l.category === "overlay").length, 0);
  const basemapCount = activeServices.reduce((n, s) => n + (layers[s.id] ?? []).filter((l) => l.category === "basemap").length, 0);

  const hasAnyStyle = displayLayers.some((l) => l.style_config != null);

  const handleFilterChange = (value: RegionFilter) => {
    setRegionFilter(value);
    if (selectedProvider && !matchesFilter(selectedProvider, value)) setSelectedProvider(null);
  };

  const toggleEnabled = (layerId: string, serviceId: string) => {
    setLayers((prev) => ({
      ...prev,
      [serviceId]: (prev[serviceId] ?? []).map((l) =>
        l.id === layerId ? { ...l, enabled: !l.enabled } : l,
      ),
    }));
  };

  return (
    <>
      {/* Page Header */}
      <div className="mb-6 flex justify-between items-start gap-2">
        <div>
          <h1 className="text-2xl font-semibold mb-1">Data Providers</h1>
          <p className="text-base-content/70">Providers, their services, and available layers.</p>
        </div>
        {isAdmin && (
          <button className="btn btn-primary gap-2">
            <Plus size={20} />
            Add Layer
          </button>
        )}
      </div>

      <div className="grid grid-cols-2 gap-6 items-start">

        {/* Left: Providers + Services */}
        <div className="card bg-base-100 border border-base-300 overflow-hidden">

          {/* Region filter + Add Provider */}
          <div className="px-4 py-2.5 border-b border-base-300 flex items-center gap-2">
            <span className="text-sm text-base-content/60 shrink-0">Region</span>
            <select
              className="select select-sm select-bordered w-full"
              value={regionFilter}
              onChange={(e) => handleFilterChange(e.target.value)}
            >
              <option value="">All regions</option>
              <option value="global">Global</option>
              <optgroup label="United Kingdom">
                <option value="GB">United Kingdom (all)</option>
                <option value="GB-ENG">England</option>
                <option value="GB-SCT">Scotland</option>
                <option value="GB-WLS">Wales</option>
                <option value="GB-NIR">Northern Ireland</option>
              </optgroup>
            </select>
            {isAdmin && (
              <button className="btn btn-sm btn-outline gap-1 shrink-0">
                <Plus size={14} />
                Add Provider
              </button>
            )}
          </div>

          {filteredProviders.length === 0 ? (
            <div className="py-12 text-center">
              <p className="text-base-content/50 text-sm">No providers for this region</p>
            </div>
          ) : (
            <div>
              {filteredProviders.map((provider) => {
                const services = MOCK_SERVICES.filter(
                  (s) => s.provider_id === provider.id,
                );
                const layerCount = services.reduce((n, s) => n + (MOCK_LAYERS[s.id] ?? []).length, 0);
                const isOpen = selectedProvider?.id === provider.id;

                return (
                  <div
                    key={`provider-${provider.id}`}
                    className={`collapse rounded-none border-b border-base-300 ${isOpen ? "collapse-open" : ""}`}
                  >
                    {/* Provider header — clicking opens accordion + selects provider */}
                    <div
                      className={`collapse-title flex items-center justify-between gap-2 py-2.5 px-4 cursor-pointer min-h-0 ${isOpen ? "bg-primary/10" : "bg-base-200 hover:bg-base-200/80"}`}
                      onClick={() => setSelectedProvider(isOpen ? null : provider)}
                    >
                      <div className="flex items-center gap-2">
                        <span className={`text-sm ${isOpen ? "font-bold" : "font-semibold"}`}>
                          {provider.name}
                        </span>
                        <span className="text-xs text-base-content/40">({layerCount})</span>
                        <RegionBadge countryCode={provider.country_code} subdivision={provider.subdivision} />
                      </div>
                      {isAdmin && (
                        <div
                          className="flex items-center gap-1 shrink-0 relative z-10"
                          onClick={(e) => e.stopPropagation()}
                        >
                          <button className="btn btn-ghost btn-xs gap-1" title="Add service">
                            <Plus size={12} />
                            <span className="text-xs">Service</span>
                          </button>
                          <button className="btn btn-ghost btn-xs" title="Edit provider">
                            <Pencil size={13} />
                          </button>
                          <button className="btn btn-ghost btn-xs text-error" title="Delete provider">
                            <Trash2 size={13} />
                          </button>
                        </div>
                      )}
                    </div>

                    {/* Services — shown when accordion is open */}
                    <div className="collapse-content px-0 pb-0">
                      {services.length === 0 ? (
                        <p className="px-8 py-2 text-xs text-base-content/40 italic">No services added</p>
                      ) : (
                        services.map((service) => (
                          <div
                            key={service.id}
                            className="flex items-center justify-between gap-2 px-4 pl-8 py-2 border-t border-base-200 hover:bg-base-50"
                          >
                            <div className="flex items-center gap-2 min-w-0">
                              <Globe size={13} className="opacity-40 shrink-0" />
                              <div className="min-w-0">
                                <div className="text-sm text-base-content/70">{service.name}</div>
                                <div className="text-xs text-base-content/40 truncate max-w-52">{service.base_url}</div>
                              </div>
                            </div>
                            <div className="flex items-center gap-2 shrink-0">
                              <span className={`badge badge-sm badge-outline ${SERVICE_TYPE_BADGE[service.service_type]}`}>
                                {service.service_type}
                              </span>
                              {isAdmin && (
                                <div className="flex gap-1">
                                  <button className="btn btn-ghost btn-xs" title="Edit service">
                                    <Pencil size={13} />
                                  </button>
                                  <button className="btn btn-ghost btn-xs text-error" title="Delete service">
                                    <Trash2 size={13} />
                                  </button>
                                </div>
                              )}
                            </div>
                          </div>
                        ))
                      )}
                    </div>
                  </div>
                );
              })}
            </div>
          )}
        </div>

        {/* Right: Layers */}
        <div className="card bg-base-100 border border-base-300 overflow-hidden">
          {/* Category tabs */}
          <div className="flex border-b border-base-300">
            <button
              className={`flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-medium transition-colors
                ${category === "overlay" ? "border-b-2 border-primary text-primary" : "text-base-content/50 hover:text-base-content"}`}
              onClick={() => setCategory("overlay")}
            >
              <Layers size={15} />
              Overlays
              <span className="text-xs opacity-60">({overlayCount})</span>
            </button>
            <button
              className={`flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-medium transition-colors
                ${category === "basemap" ? "border-b-2 border-primary text-primary" : "text-base-content/50 hover:text-base-content"}`}
              onClick={() => setCategory("basemap")}
            >
              <Map size={15} />
              Base Maps
              <span className="text-xs opacity-60">({basemapCount})</span>
            </button>
          </div>

          {/* Header */}
          <div className="flex items-center justify-between px-4 py-3 border-b border-base-300">
            <div>
              {selectedProvider ? (
                <div className="flex items-center gap-2">
                  <span className="font-medium">{selectedProvider.name}</span>
                  <RegionBadge countryCode={selectedProvider.country_code} subdivision={selectedProvider.subdivision} />
                  {providerServices.map((s) => (
                    <span key={s.id} className={`badge badge-sm badge-outline ${SERVICE_TYPE_BADGE[s.service_type]}`}>
                      {s.service_type}
                    </span>
                  ))}
                </div>
              ) : (
                <div className="font-medium">All providers</div>
              )}
            </div>
          </div>

          {displayLayers.length === 0 ? (
            <div className="card-body items-center text-center py-16">
              <MapPin size={40} className="opacity-20 mb-2" />
              <p className="text-base-content/50 text-sm">No layers added yet</p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="table table-sm">
                <thead>
                  <tr>
                    <th>Name</th>
                    {!selectedProvider && <th>Provider</th>}
                    <th>Service</th>
                    <th>Identifier</th>
                    <th>Zoom</th>
                    {hasAnyStyle && <th>Style</th>}
                    <th>Enabled</th>
                    {isAdmin && <th />}
                  </tr>
                </thead>
                <tbody>
                  {displayLayers.map((layer) => (
                    <tr key={layer.id} className="hover">
                      <td>
                        <div className="font-medium text-sm">{layer.name}</div>
                        {layer.description && (
                          <div className="text-xs text-base-content/50">{layer.description}</div>
                        )}
                      </td>
                      {!selectedProvider && (
                        <td>
                          <button
                            className="text-sm text-base-content/70 hover:text-base-content underline-offset-2 hover:underline"
                            onClick={() => setSelectedProvider(layer.provider)}
                          >
                            {layer.provider.name}
                          </button>
                        </td>
                      )}
                      <td>
                        <span className={`badge badge-sm badge-outline ${SERVICE_TYPE_BADGE[layer.service.service_type]}`}>
                          {layer.service.service_type}
                        </span>
                      </td>
                      <td>
                        <code className="text-xs bg-base-200 px-1.5 py-0.5 rounded">
                          {layer.layer_identifier}
                        </code>
                      </td>
                      <td>
                        <ZoomBadge minZoom={layer.min_zoom} maxZoom={layer.max_zoom} />
                      </td>
                      {hasAnyStyle && (
                        <td>
                          <ColourSwatch styleConfig={layer.style_config} />
                        </td>
                      )}
                      <td>
                        <input
                          type="checkbox"
                          className="toggle toggle-sm toggle-primary"
                          checked={layer.enabled}
                          onChange={() => toggleEnabled(layer.id, layer.service_id)}
                        />
                      </td>
                      {isAdmin && (
                        <td>
                          <div className="flex gap-1">
                            <button className="btn btn-ghost btn-xs" title="Edit layer">
                              <Pencil size={14} />
                            </button>
                            <button className="btn btn-ghost btn-xs text-error" title="Delete layer">
                              <Trash2 size={14} />
                            </button>
                          </div>
                        </td>
                      )}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>
    </>
  );
}
