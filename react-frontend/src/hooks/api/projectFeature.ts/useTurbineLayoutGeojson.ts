import { useApiRequest } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";

export function useTurbineLayoutGeojson(
  projectSlug: string,
  collectionSlug: string,
  featureId: number,
  enabled: boolean,
) {
  const apiRequest = useApiRequest();
  const url = `${__URLS__.api.base}${__URLS__.api.project_features}/${projectSlug}/${collectionSlug}/${featureId}?format=geojson`;
  return useQuery({
    queryKey: ["turbine-layout-geojson", projectSlug, collectionSlug, featureId],
    queryFn: () => apiRequest<GeoJSON.FeatureCollection>(url),
    enabled,
  });
}
