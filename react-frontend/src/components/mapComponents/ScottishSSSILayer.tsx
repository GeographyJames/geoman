import { useArcGISFeatureLayer } from "@/hooks/useArcGISFeatureLayer";
import { Style, Fill, Stroke } from "ol/style";

const SCOTTISH_SSSI_URL =
  "https://services1.arcgis.com/LM9GyVFsughzHdbO/ArcGIS/rest/services/Sites_of_Special_Scientific_Interest/FeatureServer/0";

const style = new Style({
  fill: new Fill({ color: "rgba(220, 38, 38, 0.15)" }),
  stroke: new Stroke({ color: "#DC2626", width: 1 }),
});

export default function ScottishSSSILayer({ visible, onLoadingChange, onFeatureCountChange, onFeatureHover }: {
  visible: boolean;
  onLoadingChange?: (loading: boolean) => void;
  onFeatureCountChange?: (count: number) => void;
  onFeatureHover?: (name: string | null, pixel: [number, number] | null) => void;
}) {
  useArcGISFeatureLayer(SCOTTISH_SSSI_URL, style, { minZoom: 12, onLoadingChange, onFeatureCountChange, onFeatureHover, visible });
  return null;
}
