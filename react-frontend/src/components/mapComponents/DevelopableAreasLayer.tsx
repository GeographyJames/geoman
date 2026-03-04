import { useVectorTileLayer } from "@/hooks/useVectorTileLayer";
import { Style, Fill, Stroke } from "ol/style";

const DEVELOPABLE_AREAS_URL = "/api/tiles/Postgres-Prod/developable_areas_uk_2024/{z}/{x}/{-y}";

const style = new Style({
  fill: new Fill({ color: "rgba(37, 99, 235, 0.2)" }),
  stroke: new Stroke({ color: "#2563EB", width: 1 }),
});

export default function DevelopableAreasLayer({ visible }: { visible: boolean }) {
  useVectorTileLayer(DEVELOPABLE_AREAS_URL, style, visible);
  return null;
}
