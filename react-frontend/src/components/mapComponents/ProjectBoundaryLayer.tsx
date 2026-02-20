import { useEffect, useRef } from "react";
import { useSearch } from "@tanstack/react-router";
import { useProjects } from "@/hooks/api/projects/useProjects";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { useProjectCollections } from "@/hooks/api/useProjectCollections";
import { useProjectCollectionItems } from "@/hooks/api/useProjectCollectionItems";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import GeoJSON from "ol/format/GeoJSON";
import { Stroke, Fill, Style } from "ol/style";

const SITE_BOUNDARIES_TITLE = "site boundaries";

const boundaryStyle = new Style({
  stroke: new Stroke({
    color: "#2A81CB",
    width: 2.5,
  }),
  fill: new Fill({
    color: "rgba(42, 129, 203, 0.12)",
  }),
});

export default function ProjectBoundaryLayer() {
  const { projects: projectsParam } = useSearch({ from: "/_app/" });
  const { data: allProjects } = useProjects();

  const slugs = projectsParam ? projectsParam.split(",") : [];
  const selectedProjects = allProjects?.filter((p) => slugs.includes(p.slug)) ?? [];

  return (
    <>
      {selectedProjects.map((p) => (
        <ProjectBoundary key={p.id} projectId={p.id} />
      ))}
    </>
  );
}

function ProjectBoundary({ projectId }: { projectId: number }) {
  const { mapRef } = useMapContext();
  const layerRef = useRef<VectorLayer | null>(null);

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

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !primaryFeature) return;

    const format = new GeoJSON();
    const olFeatures = format.readFeatures(
      { type: "FeatureCollection", features: [primaryFeature] },
      { featureProjection: "EPSG:3857", dataProjection: "EPSG:4326" },
    );

    const source = new VectorSource({ features: olFeatures });
    const layer = new VectorLayer({
      source,
      style: boundaryStyle,
    });

    map.addLayer(layer);
    layerRef.current = layer;

    const extent = source.getExtent();
    map.getView().fit(extent, { padding: [80, 80, 80, 80], maxZoom: 16, duration: 500 });

    return () => {
      if (map && layerRef.current) {
        map.removeLayer(layerRef.current);
        layerRef.current = null;
      }
    };
  }, [mapRef, primaryFeature]);

  return null;
}
