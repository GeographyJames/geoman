import { useMemo } from "react";
import { useVectorTileLayer } from "@/hooks/useVectorTileLayer";
import { Style, Fill, Stroke } from "ol/style";
import type { DataProviderLayer } from "@/domain/data_provider/types";

interface MvtSource {
  url: string;
}

interface MvtStyleConfig {
  fillColor?: string;
  fillOpacity?: number;
  strokeColor?: string;
  strokeWidth?: number;
}

function hexToRgba(hex: string, opacity: number): string {
  const h = hex.replace("#", "");
  const r = parseInt(h.slice(0, 2), 16);
  const g = parseInt(h.slice(2, 4), 16);
  const b = parseInt(h.slice(4, 6), 16);
  return `rgba(${r}, ${g}, ${b}, ${opacity})`;
}

function buildStyle(styleConfig: unknown): Style {
  const cfg = (styleConfig && typeof styleConfig === "object" ? styleConfig : {}) as MvtStyleConfig;
  const fillColor = cfg.fillColor ?? "#3B82F6";
  const fillOpacity = cfg.fillOpacity ?? 0.2;
  const strokeColor = cfg.strokeColor ?? fillColor;
  const strokeWidth = cfg.strokeWidth ?? 1;
  return new Style({
    fill: new Fill({ color: hexToRgba(fillColor, fillOpacity) }),
    stroke: new Stroke({ color: strokeColor, width: strokeWidth }),
  });
}

export default function DynamicMVTLayer({
  layer,
  visible,
}: {
  layer: DataProviderLayer;
  visible: boolean;
}) {
  const url = (layer.source as MvtSource).url;
  const style = useMemo(() => buildStyle(layer.style_config), [layer.style_config]);
  useVectorTileLayer(url, style, visible);
  return null;
}
