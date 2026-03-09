import { useState } from "react";
import { Globe, Plus, Pencil, Trash2, Layers, Map, MapPin } from "lucide-react";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";
import { useDataProviders } from "@/hooks/api/useDataProviders";
import { useDataProviderServices } from "@/hooks/api/useDataProviderServices";
import { useDataProviderLayers } from "@/hooks/api/useDataProviderLayers";
import { usePatchDataProviderLayer } from "@/hooks/api/usePatchDataProviderLayer";
import {
  CreateProviderForm,
  openCreateProviderModal,
} from "./CreateProviderForm";
import { EditProviderForm, openEditProviderModal } from "./EditProviderForm";
import {
  DeleteProviderForm,
  openDeleteProviderModal,
} from "./DeleteProviderForm";
import { CreateServiceForm, openCreateServiceModal } from "./CreateServiceForm";
import { EditServiceForm, openEditServiceModal } from "./EditServiceForm";
import { DeleteServiceForm, openDeleteServiceModal } from "./DeleteServiceForm";
import { CreateLayerForm, openCreateLayerModal } from "./CreateLayerForm";
import { EditLayerForm, openEditLayerModal } from "./EditLayerForm";
import { DeleteLayerForm, openDeleteLayerModal } from "./DeleteLayerForm";
import type {
  DataProvider,
  DataProviderService,
  DataProviderLayer,
  DataProviderServiceType,
  LayerCategory,
} from "@/domain/data_provider/types";

type LayerRow = DataProviderLayer & {
  service: DataProviderService;
  provider: DataProvider;
};

const SERVICE_TYPE_BADGE: Record<DataProviderServiceType, string> = {
  ImageWMS: "badge-info",
  TileWMS: "badge-info",
  WMTS: "badge-warning",
  WFS: "badge-success",
  ArcGISRest: "badge-secondary",
  MVT: "badge-primary",
  OGCAPIFeatures: "badge-success",
  XYZ: "badge-accent",
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

function RegionBadge({
  countryCode,
  subdivision,
}: {
  countryCode: string | null;
  subdivision: string | null;
}) {
  if (!countryCode)
    return <span className="badge badge-ghost badge-sm text-xs">Global</span>;
  if (subdivision)
    return (
      <span className="badge badge-ghost badge-sm text-xs">
        {SUBDIVISION_LABELS[subdivision] ?? subdivision}
      </span>
    );
  return (
    <span className="badge badge-ghost badge-sm text-xs">
      {COUNTRY_LABELS[countryCode] ?? countryCode}
    </span>
  );
}

function StyleSwatch({ styleConfig }: { styleConfig: unknown }) {
  if (!styleConfig || typeof styleConfig !== "object")
    return <span className="text-base-content/30">—</span>;
  const cfg = styleConfig as { fillColor?: string; fillOpacity?: number; strokeColor?: string };
  if (!cfg.fillColor && !cfg.strokeColor)
    return <span className="text-base-content/30">—</span>;
  const fillColor = cfg.fillColor ?? "#3388ff";
  const fillOpacity = cfg.fillOpacity ?? 0.2;
  const strokeColor = cfg.strokeColor ?? fillColor;
  const r = parseInt(fillColor.slice(1, 3), 16);
  const g = parseInt(fillColor.slice(3, 5), 16);
  const b = parseInt(fillColor.slice(5, 7), 16);
  return (
    <svg width="24" height="16" className="shrink-0">
      <rect
        x="1"
        y="1"
        width="22"
        height="14"
        rx="2"
        fill={`rgba(${r}, ${g}, ${b}, ${fillOpacity})`}
        stroke={strokeColor}
        strokeWidth="1.5"
      />
    </svg>
  );
}

export default function DataProvidersSection() {
  const { data: currentUser } = useCurrentUser();
  const { data: providers = [] } = useDataProviders();
  const { data: services = [] } = useDataProviderServices();
  const { data: layers = [] } = useDataProviderLayers();
  const { mutate: patchLayer } = usePatchDataProviderLayer();

  const [selectedProvider, setSelectedProvider] = useState<DataProvider | null>(
    null,
  );
  const [regionFilter, setRegionFilter] = useState<RegionFilter>("");
  const [category, setCategory] = useState<LayerCategory>("overlay");

  const [creatingServiceForProvider, setCreatingServiceForProvider] =
    useState<DataProvider | null>(null);
  const [editingProvider, setEditingProvider] = useState<DataProvider | null>(
    null,
  );
  const [deletingProvider, setDeletingProvider] = useState<DataProvider | null>(
    null,
  );
  const [editingService, setEditingService] =
    useState<DataProviderService | null>(null);
  const [deletingService, setDeletingService] =
    useState<DataProviderService | null>(null);
  const [editingLayer, setEditingLayer] = useState<DataProviderLayer | null>(
    null,
  );
  const [deletingLayer, setDeletingLayer] = useState<DataProviderLayer | null>(
    null,
  );

  const isAdmin = currentUser?.isAdmin ?? false;

  const filteredProviders = providers.filter((provider) =>
    matchesFilter(provider, regionFilter),
  );

  const handleFilterChange = (value: RegionFilter) => {
    setRegionFilter(value);
    if (selectedProvider && !matchesFilter(selectedProvider, value))
      setSelectedProvider(null);
  };

  const activeServices = selectedProvider
    ? services.filter((s) => s.provider_id === selectedProvider.id)
    : services.filter((s) =>
        filteredProviders.some((p) => p.id === s.provider_id),
      );

  const displayLayers: LayerRow[] = activeServices.flatMap((service) => {
    const provider = providers.find((p) => p.id === service.provider_id);
    if (!provider) return [];
    return layers
      .filter((l) => l.service_id === service.id && l.category === category)
      .map((l) => ({ ...l, service, provider }));
  });

  const overlayCount = activeServices.reduce(
    (n, s) =>
      n +
      layers.filter((l) => l.service_id === s.id && l.category === "overlay")
        .length,
    0,
  );
  const basemapCount = activeServices.reduce(
    (n, s) =>
      n +
      layers.filter((l) => l.service_id === s.id && l.category === "basemap")
        .length,
    0,
  );

  const hasAnyStyle = displayLayers.some(
    (l) =>
      l.style_config &&
      typeof l.style_config === "object" &&
      Object.keys(l.style_config as object).length > 0,
  );

  return (
    <>
      <CreateProviderForm />
      <EditProviderForm
        provider={editingProvider}
        onClose={() => setEditingProvider(null)}
      />
      <DeleteProviderForm
        provider={deletingProvider}
        onClose={() => setDeletingProvider(null)}
      />
      <CreateServiceForm provider={creatingServiceForProvider} />
      <EditServiceForm
        service={editingService}
        onClose={() => setEditingService(null)}
      />
      <DeleteServiceForm
        service={deletingService}
        onClose={() => setDeletingService(null)}
      />
      <CreateLayerForm />
      <EditLayerForm
        layer={editingLayer}
        onClose={() => setEditingLayer(null)}
      />
      <DeleteLayerForm
        layer={deletingLayer}
        onClose={() => setDeletingLayer(null)}
      />

      {/* Page Header */}
      <div className="mb-6 flex justify-between items-start gap-2">
        <div>
          <h1 className="text-2xl font-semibold mb-1">Data Providers</h1>
          <p className="text-base-content/70">
            Providers, their services, and available layers.
          </p>
        </div>
        {isAdmin && (
          <button
            type="button"
            className="btn btn-primary gap-2"
            onClick={openCreateLayerModal}
          >
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
            <span className="text-sm text-base-content/60 shrink-0">
              Region
            </span>
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
              <button
                type="button"
                className="btn btn-sm btn-outline gap-1 shrink-0"
                onClick={openCreateProviderModal}
              >
                <Plus size={14} />
                Add Provider
              </button>
            )}
          </div>

          {filteredProviders.length === 0 ? (
            <div className="py-12 text-center">
              <p className="text-base-content/50 text-sm">
                No providers for this region
              </p>
            </div>
          ) : (
            <div>
              {filteredProviders.map((provider) => {
                const providerServices = services.filter(
                  (s) => s.provider_id === provider.id,
                );
                const serviceCount = providerServices.length;
                const layerCount = providerServices.reduce(
                  (n, s) =>
                    n + layers.filter((l) => l.service_id === s.id).length,
                  0,
                );
                const isOpen = selectedProvider?.id === provider.id;

                return (
                  <div
                    key={`provider-${provider.id}`}
                    className={`collapse rounded-none border-b border-base-300 ${isOpen ? "collapse-open" : ""}`}
                  >
                    <div
                      className={`collapse-title flex items-center justify-between gap-2 py-2.5 px-4 cursor-pointer min-h-0 ${isOpen ? "bg-primary/10" : "bg-base-200 hover:bg-base-200/80"}`}
                      onClick={() =>
                        setSelectedProvider(isOpen ? null : provider)
                      }
                    >
                      <div className="flex items-center gap-2">
                        <span
                          className={`text-sm ${isOpen ? "font-bold" : "font-semibold"}`}
                        >
                          {provider.name}
                        </span>
                        <span className="text-xs text-base-content/40">
                          {serviceCount}s / {layerCount}l
                        </span>
                        <RegionBadge
                          countryCode={provider.country_code}
                          subdivision={provider.subdivision}
                        />
                      </div>
                      {isAdmin && (
                        <div
                          className="flex items-center gap-1 shrink-0 relative z-10"
                          onClick={(e) => e.stopPropagation()}
                        >
                          <button
                            type="button"
                            className="btn btn-ghost btn-xs gap-1"
                            title="Add service"
                            onClick={() => {
                              setCreatingServiceForProvider(provider);
                              openCreateServiceModal();
                            }}
                          >
                            <Plus size={12} />
                            <span className="text-xs">Service</span>
                          </button>
                          <button
                            type="button"
                            className="btn btn-ghost btn-xs"
                            title="Edit provider"
                            onClick={() => {
                              setEditingProvider(provider);
                              openEditProviderModal();
                            }}
                          >
                            <Pencil size={13} />
                          </button>
                          <button
                            type="button"
                            className="btn btn-ghost btn-xs text-error"
                            title="Delete provider"
                            onClick={() => {
                              setDeletingProvider(provider);
                              openDeleteProviderModal();
                            }}
                          >
                            <Trash2 size={13} />
                          </button>
                        </div>
                      )}
                    </div>

                    <div className="collapse-content px-0 pb-0">
                      {providerServices.length === 0 ? (
                        <p className="px-8 py-2 text-xs text-base-content/40 italic">
                          No services added
                        </p>
                      ) : (
                        providerServices.map((service) => (
                          <div
                            key={service.id}
                            className="flex items-center justify-between gap-2 px-4 pl-8 py-2 border-t border-base-200 hover:bg-base-50"
                          >
                            <div className="flex items-center gap-2 min-w-0">
                              <Globe
                                size={13}
                                className="opacity-40 shrink-0"
                              />
                              <div className="min-w-0">
                                <div className="text-sm text-base-content/70">
                                  {service.name}
                                </div>
                                {service.base_url && (
                                  <div className="text-xs text-base-content/40 truncate max-w-52">
                                    {service.base_url}
                                  </div>
                                )}
                              </div>
                            </div>
                            <div className="flex items-center gap-2 shrink-0">
                              <span
                                className={`badge badge-sm badge-outline ${SERVICE_TYPE_BADGE[service.service_type]}`}
                              >
                                {service.service_type}
                              </span>
                              {isAdmin && (
                                <div className="flex gap-1">
                                  <button
                                    type="button"
                                    className="btn btn-ghost btn-xs"
                                    title="Edit service"
                                    onClick={() => {
                                      setEditingService(service);
                                      openEditServiceModal();
                                    }}
                                  >
                                    <Pencil size={13} />
                                  </button>
                                  <button
                                    type="button"
                                    className="btn btn-ghost btn-xs text-error"
                                    title="Delete service"
                                    onClick={() => {
                                      setDeletingService(service);
                                      openDeleteServiceModal();
                                    }}
                                  >
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
          {/* Header */}
          <div className="px-4 py-3 border-b border-base-300">
            {selectedProvider ? (
              <div className="flex items-center gap-2">
                <span className="font-medium">{selectedProvider.name}</span>
                <RegionBadge
                  countryCode={selectedProvider.country_code}
                  subdivision={selectedProvider.subdivision}
                />
                {services
                  .filter((s) => s.provider_id === selectedProvider.id)
                  .map((s) => (
                    <span
                      key={s.id}
                      className={`badge badge-sm badge-outline ${SERVICE_TYPE_BADGE[s.service_type]}`}
                    >
                      {s.service_type}
                    </span>
                  ))}
              </div>
            ) : (
              <div className="font-medium">All providers</div>
            )}
          </div>
          {/* Category tabs */}
          <div className="flex border-b border-base-300">
            <button
              type="button"
              className={`flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-medium transition-colors
                ${category === "overlay" ? "border-b-2 border-primary text-primary" : "text-base-content/50 hover:text-base-content"}`}
              onClick={() => setCategory("overlay")}
            >
              <Layers size={15} />
              Overlays
              <span className="text-xs opacity-60">({overlayCount})</span>
            </button>
            <button
              type="button"
              className={`flex-1 flex items-center justify-center gap-2 py-2.5 text-sm font-medium transition-colors
                ${category === "basemap" ? "border-b-2 border-primary text-primary" : "text-base-content/50 hover:text-base-content"}`}
              onClick={() => setCategory("basemap")}
            >
              <Map size={15} />
              Base Maps
              <span className="text-xs opacity-60">({basemapCount})</span>
            </button>
          </div>
          {displayLayers.length === 0 ? (
            <div className="card-body items-center text-center py-16">
              <MapPin size={40} className="opacity-20 mb-2" />
              <p className="text-base-content/50 text-sm">
                No layers added yet
              </p>
            </div>
          ) : (
            <div className="overflow-x-auto">
              <table className="table table-sm">
                <thead>
                  <tr>
                    <th>Name</th>
                    {!selectedProvider && <th>Provider</th>}
                    <th>Service</th>
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
                          <div className="text-xs text-base-content/50">
                            {layer.description}
                          </div>
                        )}
                      </td>
                      {!selectedProvider && (
                        <td>
                          <button
                            type="button"
                            className="text-sm text-base-content/70 hover:text-base-content underline-offset-2 hover:underline"
                            onClick={() => setSelectedProvider(layer.provider)}
                          >
                            {layer.provider.name}
                          </button>
                        </td>
                      )}
                      <td>
                        <span
                          className={`badge badge-sm badge-outline ${SERVICE_TYPE_BADGE[layer.service.service_type]}`}
                        >
                          {layer.service.service_type}
                        </span>
                      </td>
                      {hasAnyStyle && (
                        <td>
                          <StyleSwatch styleConfig={layer.style_config} />
                        </td>
                      )}
                      <td>
                        <input
                          type="checkbox"
                          className="toggle toggle-sm toggle-primary"
                          checked={layer.enabled}
                          onChange={() =>
                            patchLayer({
                              id: layer.id,
                              patch: { enabled: !layer.enabled },
                            })
                          }
                        />
                      </td>
                      {isAdmin && (
                        <td>
                          <div className="flex gap-1">
                            <button
                              type="button"
                              className="btn btn-ghost btn-xs"
                              title="Edit layer"
                              onClick={() => {
                                setEditingLayer(layer);
                                openEditLayerModal();
                              }}
                            >
                              <Pencil size={14} />
                            </button>
                            <button
                              type="button"
                              className="btn btn-ghost btn-xs text-error"
                              title="Delete layer"
                              onClick={() => {
                                setDeletingLayer(layer);
                                openDeleteLayerModal();
                              }}
                            >
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
