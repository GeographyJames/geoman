import { useEffect } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorTileLayer from "ol/layer/VectorTile";
import VectorTileSource from "ol/source/VectorTile";
import MVT from "ol/format/MVT";
import type { StyleLike } from "ol/style/Style";

/** Adds an MVT vector tile layer to the shared map. Adds on mount, removes on unmount. */
export function useVectorTileLayer(url: string, style?: StyleLike) {
  const { mapRef } = useMapContext();

  useEffect(() => {
    const map = mapRef.current;
    if (!map) return;

    const layer = new VectorTileLayer({
      source: new VectorTileSource({
        format: new MVT(),
        url,
      }),
      style,
    });

    map.getLayers().insertAt(1, layer);

    return () => {
      map.removeLayer(layer);
    };
  }, [mapRef, url]);
}
