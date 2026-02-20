import { useCallback, useEffect } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { useProjects } from "@/hooks/api/projects/useProjects";
import VectorSource from "ol/source/Vector";
import GeoJSON from "ol/format/GeoJSON";
import { fromLonLat } from "ol/proj";
import { boundingExtent } from "ol/extent";

function readExtent(feature: GeoJSON.Feature): import("ol/extent").Extent {
  const format = new GeoJSON();
  const olFeatures = format.readFeatures(
    { type: "FeatureCollection", features: [feature] },
    { featureProjection: "EPSG:3857", dataProjection: "EPSG:4326" },
  );
  return new VectorSource({ features: olFeatures }).getExtent();
}

/** Zooms to a boundary feature on mount, back to all projects on unmount.
 *  Returns `zoomToProject` callback and `hasExtent` flag. */
export function useZoomToProjectBoundary(feature: GeoJSON.Feature | undefined) {
  const { mapRef } = useMapContext();
  const { data: allProjects } = useProjects();

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !feature) return;

    const extent = readExtent(feature);
    map.getView().fit(extent, { padding: [80, 80, 80, 80], maxZoom: 16, duration: 500 });

    return () => {
      if (!map || !allProjects) return;

      const coordinates = allProjects
        .filter((p) => p.centroid)
        .map((p) => fromLonLat(p.centroid!.coordinates));

      if (coordinates.length > 0) {
        const allExtent = boundingExtent(coordinates);
        map.getView().fit(allExtent, { padding: [50, 50, 50, 50], maxZoom: 16, duration: 500 });
      }
    };
  }, [mapRef, feature]);

  const zoomToProject = useCallback(() => {
    const map = mapRef.current;
    if (!map || !feature) return;

    const extent = readExtent(feature);
    map.getView().fit(extent, { padding: [80, 80, 80, 80], maxZoom: 16, duration: 500 });
  }, [mapRef, feature]);

  return { zoomToProject, hasExtent: !!feature };
}
