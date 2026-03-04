import { useEffect, useRef } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import TileLayer from "ol/layer/Tile";
import TileWMS from "ol/source/TileWMS";

/** Adds a WMS tile layer to the shared map. Adds on mount, removes on unmount. Visibility is toggled via setVisible. */
export function useWmsLayer(url: string, layer: string, visible = true) {
  const { mapRef } = useMapContext();
  const layerRef = useRef<TileLayer<TileWMS> | null>(null);

  useEffect(() => {
    const map = mapRef.current;
    if (!map) return;

    const wmsLayer = new TileLayer({
      source: new TileWMS({
        url,
        params: { LAYERS: layer, TILED: true },
        serverType: "geoserver",
      }),
      visible,
    });

    map.getLayers().insertAt(1, wmsLayer);
    layerRef.current = wmsLayer;

    return () => {
      map.removeLayer(wmsLayer);
      layerRef.current = null;
    };
  }, [mapRef, url, layer]);

  useEffect(() => {
    layerRef.current?.setVisible(visible);
  }, [visible]);
}
