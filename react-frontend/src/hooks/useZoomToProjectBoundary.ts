import { useCallback, useEffect, useRef } from "react";
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
  const hasZoomedRef = useRef(false);
  const allProjectsRef = useRef(allProjects);

  useEffect(() => {
    allProjectsRef.current = allProjects;
  });

  // Zoom to project only the first time a feature becomes available
  useEffect(() => {
    const map = mapRef.current;
    if (!map || !feature || hasZoomedRef.current) return;

    hasZoomedRef.current = true;
    const extent = readExtent(feature);
    map.getView().fit(extent, { padding: [80, 80, 80, 80], maxZoom: 16, duration: 500 });
  }, [mapRef, feature]);

  // Zoom to all projects only on unmount
  useEffect(() => {
    return () => {
      const map = mapRef.current;
      const projects = allProjectsRef.current;
      if (!map || !projects) return;

      const coordinates = projects
        .filter((p) => p.centroid)
        .map((p) => fromLonLat(p.centroid!.coordinates));

      if (coordinates.length > 0) {
        const allExtent = boundingExtent(coordinates);
        map.getView().fit(allExtent, { padding: [50, 50, 50, 50], maxZoom: 16, duration: 500 });
      }
    };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const zoomToProject = useCallback(() => {
    const map = mapRef.current;
    if (!map || !feature) return;

    const extent = readExtent(feature);
    map.getView().fit(extent, { padding: [80, 80, 80, 80], maxZoom: 16, duration: 500 });
  }, [mapRef, feature]);

  return { zoomToProject, hasExtent: !!feature };
}
