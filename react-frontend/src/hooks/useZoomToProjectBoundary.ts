import { useCallback } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { useProjectCollections } from "@/hooks/api/useProjectCollections";
import { useProjectCollectionItems } from "@/hooks/api/useProjectCollectionItems";
import VectorSource from "ol/source/Vector";
import GeoJSON from "ol/format/GeoJSON";

const SITE_BOUNDARIES_TITLE = "site boundaries";

export function useZoomToProjectBoundary(projectId: number) {
  const { mapRef } = useMapContext();
  const { data: collectionsData } = useProjectCollections({ projectId });

  const siteBoundariesCollection = collectionsData?.collections.find(
    (c) => c.title.toLowerCase() === SITE_BOUNDARIES_TITLE,
  );

  const { data: itemsData } = useProjectCollectionItems({
    projectId,
    collectionId: siteBoundariesCollection?.id ?? "",
    enabled: !!siteBoundariesCollection,
  });

  const primaryFeature = itemsData?.features.find(
    (f) => f.properties.is_primary,
  );

  const zoomToProject = useCallback(() => {
    const map = mapRef.current;
    if (!map || !primaryFeature) return;

    const format = new GeoJSON();
    const olFeatures = format.readFeatures(
      { type: "FeatureCollection", features: [primaryFeature] },
      { featureProjection: "EPSG:3857", dataProjection: "EPSG:4326" },
    );

    const source = new VectorSource({ features: olFeatures });
    const extent = source.getExtent();
    map.getView().fit(extent, { padding: [80, 80, 80, 80], maxZoom: 16, duration: 500 });
  }, [mapRef, primaryFeature]);

  return { zoomToProject, hasExtent: !!primaryFeature };
}
