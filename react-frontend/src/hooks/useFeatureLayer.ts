import { useCallback, useEffect, useRef } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import GeoJSON from "ol/format/GeoJSON";
import type { StyleLike } from "ol/style/Style";

function readExtent(feature: GeoJSON.Feature): import("ol/extent").Extent {
  const format = new GeoJSON();
  const olFeatures = format.readFeatures(
    { type: "FeatureCollection", features: [feature] },
    { featureProjection: "EPSG:3857", dataProjection: "EPSG:4326" },
  );
  return new VectorSource({ features: olFeatures }).getExtent();
}

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

/** Returns a callback that zooms the map to fit the given GeoJSON feature. */
export function useZoomToFeature(feature: GeoJSON.Feature | undefined) {
  const { mapRef } = useMapContext();

  return useCallback(() => {
    const map = mapRef.current;
    if (!map || !feature) return;

    const extent = readExtent(feature);
    map.getView().fit(extent, { padding: [80, 80, 80, 80], maxZoom: 16, duration: 500 });
  }, [mapRef, feature]);
}
