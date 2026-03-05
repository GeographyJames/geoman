import { useState } from "react";
import DynamicMVTLayer from "@/components/mapComponents/DynamicMVTLayer";
import SSSILayer from "@/components/mapComponents/SSSILayer";
import ScottishSSSILayer from "@/components/mapComponents/ScottishSSSILayer";
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
  const [showScottishSSSI, setShowScottishSSSI] = useState(false);
  const [scottishSSSILoading, setScottishSSSILoading] = useState(false);
  const [scottishSSSICount, setScottishSSSICount] = useState(0);
  const [hoveredName, setHoveredName] = useState<string | null>(null);
  const [hoverPixel, setHoverPixel] = useState<[number, number] | null>(null);
  const [dynamicVisibility, setDynamicVisibility] = useState<
    Record<number, boolean>
  >({});

  const { data: allLayers = [] } = useDataProviderLayers();
  const { data: services = [] } = useDataProviderServices();

  const mvtServiceIds = new Set(
    services.filter((s) => s.service_type === "MVT").map((s) => s.id),
  );
  const mvtOverlays = allLayers.filter(
    (l) =>
      l.enabled && l.category === "overlay" && mvtServiceIds.has(l.service_id),
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
      <SSSILayer visible={showSSSI} />
      <ScottishSSSILayer
        visible={showScottishSSSI}
        onLoadingChange={setScottishSSSILoading}
        onFeatureCountChange={setScottishSSSICount}
        onFeatureHover={handleFeatureHover}
      />
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

            <label
              className={`flex items-center gap-2 cursor-pointer text-sm transition-opacity ${scottishSSSIVisible ? "opacity-100" : "opacity-40"}`}
            >
              <input
                type="checkbox"
                className="checkbox checkbox-sm"
                checked={showScottishSSSI}
                onChange={(e) => setShowScottishSSSI(e.target.checked)}
              />
              <svg width="20" height="14" className="shrink-0">
                <rect
                  x="1"
                  y="1"
                  width="18"
                  height="12"
                  rx="2"
                  fill="rgba(220, 38, 38, 0.15)"
                  stroke="#DC2626"
                  strokeWidth="1.5"
                />
              </svg>
              <span className="flex items-center gap-1">
                Scottish SSSI
                {!scottishSSSIVisible && (
                  <span className="badge badge-xs text-base-content/50 font-mono">
                    z{SCOTTISH_SSSI_MIN_ZOOM}+
                  </span>
                )}
              </span>
              {scottishSSSILoading ? (
                <span className="loading loading-spinner loading-xs ml-auto" />
              ) : (
                showScottishSSSI &&
                scottishSSSIVisible &&
                scottishSSSICount > 0 && (
                  <span className="text-xs text-base-content/50 ml-auto">
                    {scottishSSSICount.toLocaleString()}
                  </span>
                )
              )}
            </label>

            {mvtOverlays.map((layer) => (
              <label
                key={layer.id}
                className="flex items-center gap-2 cursor-pointer text-sm"
              >
                <input
                  type="checkbox"
                  className="checkbox checkbox-sm"
                  checked={dynamicVisibility[layer.id] ?? false}
                  onChange={(e) =>
                    setDynamicVisibility((prev) => ({
                      ...prev,
                      [layer.id]: e.target.checked,
                    }))
                  }
                />
                <LayerSwatch styleConfig={layer.style_config} />
                {layer.name}
              </label>
            ))}
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
