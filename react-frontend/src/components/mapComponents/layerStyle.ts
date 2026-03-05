import { Style, Fill, Stroke } from "ol/style";

interface StyleConfig {
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

export function buildLayerStyle(styleConfig: unknown): Style {
  const cfg = (styleConfig && typeof styleConfig === "object" ? styleConfig : {}) as StyleConfig;
  const fillColor = cfg.fillColor ?? "#3B82F6";
  const fillOpacity = cfg.fillOpacity ?? 0.2;
  const strokeColor = cfg.strokeColor ?? fillColor;
  const strokeWidth = cfg.strokeWidth ?? 1;
  return new Style({
    fill: new Fill({ color: hexToRgba(fillColor, fillOpacity) }),
    stroke: new Stroke({ color: strokeColor, width: strokeWidth }),
  });
}
