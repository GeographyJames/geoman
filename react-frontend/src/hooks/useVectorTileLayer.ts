import { useEffect, useRef } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorTileLayer from "ol/layer/VectorTile";
import VectorTileSource from "ol/source/VectorTile";
import MVT from "ol/format/MVT";
import type { StyleLike } from "ol/style/Style";

/** Adds an MVT vector tile layer to the shared map. Adds on mount, removes on unmount. Visibility is toggled via setVisible. */
export function useVectorTileLayer(url: string, style?: StyleLike, visible = true) {
  const { mapRef } = useMapContext();
  const layerRef = useRef<VectorTileLayer | null>(null);

  useEffect(() => {
    const map = mapRef.current;
    if (!map) return;

    const layer = new VectorTileLayer({
      source: new VectorTileSource({
        format: new MVT(),
        url,
      }),
      style,
      visible,
    });

    map.getLayers().insertAt(1, layer);
    layerRef.current = layer;

    return () => {
      map.removeLayer(layer);
      layerRef.current = null;
    };
  }, [mapRef, url]);

  useEffect(() => {
    layerRef.current?.setVisible(visible);
  }, [visible]);
}
