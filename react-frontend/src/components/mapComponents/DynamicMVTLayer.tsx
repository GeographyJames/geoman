import { useMemo } from "react";
import { useVectorTileLayer } from "@/hooks/useVectorTileLayer";
import { buildLayerStyle } from "@/components/mapComponents/layerStyle";
import type { DataProviderLayer } from "@/domain/data_provider/types";

interface MvtSource {
  url: string;
}

interface DisplayOptions {
  minZoom?: number;
  maxZoom?: number;
}

export default function DynamicMVTLayer({
  layer,
  visible,
}: {
  layer: DataProviderLayer;
  visible: boolean;
}) {
  const url = (layer.source as MvtSource).url;
  const style = useMemo(() => buildLayerStyle(layer.style_config), [layer.style_config]);
  const { minZoom, maxZoom } = (layer.display_options as DisplayOptions | null) ?? {};
  useVectorTileLayer(url, style, visible, { minZoom, maxZoom });
  return null;
}
