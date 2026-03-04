import { useWmsLayer } from "@/hooks/useWmsLayer";

const DATAMAP_WALES_WMS_URL = "https://datamap.gov.wales/geoserver/ows";
const SSSI_LAYER = "inspire-nrw:NRW_SSSI";

export default function SSSILayer({ visible }: { visible: boolean }) {
  useWmsLayer(DATAMAP_WALES_WMS_URL, SSSI_LAYER, visible);
  return null;
}
