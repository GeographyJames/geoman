import { useState } from "react";
import DynamicMVTLayer from "@/components/mapComponents/DynamicMVTLayer";
import DynamicArcGISLayer from "@/components/mapComponents/DynamicArcGISLayer";
import SSSILayer from "@/components/mapComponents/SSSILayer";

import { useMapZoom } from "@/hooks/useMapZoom";
import { useDataProviderLayers } from "@/hooks/api/useDataProviderLayers";
import { useDataProviderServices } from "@/hooks/api/useDataProviderServices";

const SCOTTISH_SSSI_MIN_ZOOM = 12;

interface MvtStyleConfig {
  fillColor?: string;
  fillOpacity?: number;
  strokeColor?: string;
}

function LayerSwatch({ styleConfig }: { styleConfig: unknown }) {
  const cfg = (
    styleConfig && typeof styleConfig === "object" ? styleConfig : {}
  ) as MvtStyleConfig;
  const fillColor = cfg.fillColor ?? "#3B82F6";
  const fillOpacity = cfg.fillOpacity ?? 0.2;
  const strokeColor = cfg.strokeColor ?? fillColor;
  const r = parseInt(fillColor.slice(1, 3), 16);
  const g = parseInt(fillColor.slice(3, 5), 16);
  const b = parseInt(fillColor.slice(5, 7), 16);
  return (
    <svg width="20" height="14" className="shrink-0">
      <rect
        x="1"
        y="1"
        width="18"
        height="12"
        rx="2"
        fill={`rgba(${r}, ${g}, ${b}, ${fillOpacity})`}
        stroke={strokeColor}
        strokeWidth="1.5"
      />
    </svg>
  );
}

function LayersIcon() {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="18"
      height="18"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <polygon points="12 2 2 7 12 12 22 7 12 2" />
      <polyline points="2 17 12 22 22 17" />
      <polyline points="2 12 12 17 22 12" />
    </svg>
  );
}

export function MapLayerControls() {
  const [panelOpen, setPanelOpen] = useState(() => window.innerWidth >= 768);
  const [showSSSI, setShowSSSI] = useState(false);

  const [hoveredName, setHoveredName] = useState<string | null>(null);
  const [hoverPixel, setHoverPixel] = useState<[number, number] | null>(null);
  const [dynamicVisibility, setDynamicVisibility] = useState<
    Record<number, boolean>
  >({});
  const [loadingLayers, setLoadingLayers] = useState<Record<number, boolean>>(
    {},
  );

  const handleLoadingChange = (layerId: number, loading: boolean) => {
    setLoadingLayers((prev) => ({ ...prev, [layerId]: loading }));
  };

  const { data: allLayers = [] } = useDataProviderLayers();
  const { data: services = [] } = useDataProviderServices();

  const serviceById = new Map(services.map((s) => [s.id, s]));

  const mvtServiceIds = new Set(
    services.filter((s) => s.service_type === "MVT").map((s) => s.id),
  );
  const mvtOverlays = allLayers.filter(
    (l) =>
      l.enabled && l.category === "overlay" && mvtServiceIds.has(l.service_id),
  );

  const arcgisServiceIds = new Set(
    services.filter((s) => s.service_type === "ArcGISRest").map((s) => s.id),
  );
  const arcgisOverlays = allLayers.filter(
    (l) =>
      l.enabled &&
      l.category === "overlay" &&
      arcgisServiceIds.has(l.service_id),
  );

  const handleFeatureHover = (
    name: string | null,
    pixel: [number, number] | null,
  ) => {
    setHoveredName(name);
    setHoverPixel(pixel);
  };
  const zoom = useMapZoom();
  const scottishSSSIVisible = (zoom ?? 0) >= SCOTTISH_SSSI_MIN_ZOOM;

  return (
    <>
      {mvtOverlays.map((layer) => (
        <DynamicMVTLayer
          key={layer.id}
          layer={layer}
          visible={dynamicVisibility[layer.id] ?? false}
        />
      ))}
      {arcgisOverlays.map((layer) => {
        const svc = serviceById.get(layer.service_id);
        if (!svc?.base_url) return null;
        return (
          <DynamicArcGISLayer
            key={layer.id}
            layer={layer}
            baseUrl={svc.base_url}
            visible={dynamicVisibility[layer.id] ?? false}
            onLoadingChange={(loading) => handleLoadingChange(layer.id, loading)}
          />
        );
      })}
      <SSSILayer visible={showSSSI} />

      {hoveredName && hoverPixel && (
        <div
          className="absolute pointer-events-none bg-base-100 text-sm px-2 py-1 rounded shadow-md border border-base-300 z-10 max-w-xs"
          style={{ left: hoverPixel[0] + 12, top: hoverPixel[1] - 8 }}
        >
          SSSI: {hoveredName}
        </div>
      )}

      <div className="absolute top-4 right-4 pointer-events-auto">
        {panelOpen ? (
          <div className="bg-base-100 rounded-box shadow-md p-3 flex flex-col gap-2">
            <div className="flex items-center justify-between gap-4">
              <span className="text-xs text-base-content/50">
                Zoom: {zoom?.toFixed(1) ?? "—"}
              </span>
              <button
                className="btn btn-ghost btn-xs btn-square"
                onClick={() => setPanelOpen(false)}
                aria-label="Close layers panel"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="14"
                  height="14"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  strokeWidth="2.5"
                  strokeLinecap="round"
                  strokeLinejoin="round"
                >
                  <line x1="18" y1="6" x2="6" y2="18" />
                  <line x1="6" y1="6" x2="18" y2="18" />
                </svg>
              </button>
            </div>

            <label className="flex items-center gap-2 cursor-pointer text-sm">
              <input
                type="checkbox"
                className="checkbox checkbox-sm"
                checked={showSSSI}
                onChange={(e) => setShowSSSI(e.target.checked)}
              />
              <img
                src="https://datamap.gov.wales/geoserver/ows?SERVICE=WMS&REQUEST=GetLegendGraphic&FORMAT=image/png&LAYER=inspire-nrw:NRW_SSSI"
                alt="SSSI legend"
                className="shrink-0"
              />
              SSSI (Wales)
            </label>

            {[...mvtOverlays, ...arcgisOverlays].map((layer) => {
              const { minZoom: lMin, maxZoom: lMax } =
                (layer.display_options as {
                  minZoom?: number;
                  maxZoom?: number;
                } | null) ?? {};
              const outOfZoom =
                (lMin !== undefined && (zoom ?? 0) < lMin) ||
                (lMax !== undefined && (zoom ?? 0) > lMax);
              const isVisible = dynamicVisibility[layer.id] ?? false;
              const isLoading = isVisible && !!loadingLayers[layer.id];
              return (
                <label
                  key={layer.id}
                  className={`flex items-center gap-2 cursor-pointer text-sm${outOfZoom ? " opacity-50" : ""}`}
                >
                  <input
                    type="checkbox"
                    className="checkbox checkbox-sm"
                    checked={isVisible}
                    onChange={(e) =>
                      setDynamicVisibility((prev) => ({
                        ...prev,
                        [layer.id]: e.target.checked,
                      }))
                    }
                  />
                  {isLoading ? (
                    <span className="loading loading-spinner loading-xs shrink-0" />
                  ) : (
                    <LayerSwatch styleConfig={layer.style_config} />
                  )}
                  <span className="flex-1">{layer.name}</span>
                  {lMin !== undefined && (
                    <span className="badge badge-xs text-base-content/50">
                      z{lMin}+
                    </span>
                  )}
                </label>
              );
            })}
          </div>
        ) : (
          <button
            className="btn btn-square btn-sm bg-base-100 shadow-md border-0"
            onClick={() => setPanelOpen(true)}
            aria-label="Open layers panel"
          >
            <LayersIcon />
          </button>
        )}
      </div>
    </>
  );
}
