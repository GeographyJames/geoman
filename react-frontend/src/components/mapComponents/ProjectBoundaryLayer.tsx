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
import { fromLonLat } from "ol/proj";
import { boundingExtent } from "ol/extent";

const SITE_BOUNDARIES_TITLE = "site boundaries";

const boundaryStyle = new Style({
  stroke: new Stroke({
    color: "#DC2626",
    width: 2.5,
  }),
  fill: new Fill({
    color: "rgba(220, 38, 38, 0.12)",
  }),
});

export default function ProjectBoundaryLayer() {
  const { projects: projectsParam } = useSearch({ from: "/_app/" });
  const { data: allProjects } = useProjects();
  const { mapRef } = useMapContext();
  const prevSlugsRef = useRef<string[]>([]);

  const slugs = projectsParam ? projectsParam.split(",").filter(Boolean) : [];
  const selectedProjects = allProjects?.filter((p) => slugs.includes(p.slug)) ?? [];

  useEffect(() => {
    const prev = prevSlugsRef.current;
    const removed = prev.length > 0 && slugs.length < prev.length;
    prevSlugsRef.current = slugs;

    if (!removed) return;

    const map = mapRef.current;
    if (!map || !allProjects) return;

    const coordinates = allProjects
      .filter((p) => p.centroid)
      .map((p) => fromLonLat(p.centroid!.coordinates));

    if (coordinates.length > 0) {
      const extent = boundingExtent(coordinates);
      map.getView().fit(extent, { padding: [50, 50, 50, 50], maxZoom: 16, duration: 500 });
    }
  }, [slugs.join(","), mapRef, allProjects]);

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

    map.getLayers().insertAt(1, layer);
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
