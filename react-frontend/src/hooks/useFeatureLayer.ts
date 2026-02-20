import { useEffect, useRef } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import GeoJSON from "ol/format/GeoJSON";
import type { StyleLike } from "ol/style/Style";

/** Draws a GeoJSON feature on the map. Adds on mount, removes on unmount. */
export function useFeatureLayer(feature: GeoJSON.Feature | undefined, style: StyleLike) {
  const { mapRef } = useMapContext();
  const layerRef = useRef<VectorLayer | null>(null);

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !feature) return;

    const format = new GeoJSON();
    const olFeatures = format.readFeatures(
      { type: "FeatureCollection", features: [feature] },
      { featureProjection: "EPSG:3857", dataProjection: "EPSG:4326" },
    );

    const source = new VectorSource({ features: olFeatures });
    const layer = new VectorLayer({ source, style });

    map.getLayers().insertAt(1, layer);
    layerRef.current = layer;

    return () => {
      if (map && layerRef.current) {
        map.removeLayer(layerRef.current);
        layerRef.current = null;
      }
    };
  }, [mapRef, feature]);
}
