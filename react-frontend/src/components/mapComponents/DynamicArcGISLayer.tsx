import { useMemo } from "react";
import { useArcGISFeatureLayer } from "@/hooks/useArcGISFeatureLayer";
import { buildLayerStyle } from "@/components/mapComponents/layerStyle";
import type { DataProviderLayer } from "@/domain/data_provider/types";

interface ArcGISSource {
  service_name: string;
  layer_id: number;
}

interface DisplayOptions {
  minZoom?: number;
  maxZoom?: number;
}

export default function DynamicArcGISLayer({
  layer,
  baseUrl,
  visible,
  onLoadingChange,
}: {
  layer: DataProviderLayer;
  baseUrl: string;
  visible: boolean;
  onLoadingChange?: (loading: boolean) => void;
}) {
  const { service_name, layer_id } = layer.source as ArcGISSource;
  const serviceUrl = `${baseUrl}/${service_name}/FeatureServer/${layer_id}`;
  const style = useMemo(() => buildLayerStyle(layer.style_config), [layer.style_config]);
  const { minZoom, maxZoom } = (layer.display_options as DisplayOptions | null) ?? {};
  useArcGISFeatureLayer(serviceUrl, style, { visible, minZoom, maxZoom, onLoadingChange });
  return null;
}
