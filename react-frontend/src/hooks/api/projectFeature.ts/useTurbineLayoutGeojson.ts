import { useApiRequest } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";

export interface TurbineProperties {
  turbine_number: number;
  hub_height_mm: number | null;
  rotor_diameter_mm: number | null;
  x_storage_crs: number;
  y_storage_crs: number;
}

export type TurbineFeature = GeoJSON.Feature<GeoJSON.Point, TurbineProperties>;

export type TurbineFeatureCollection = GeoJSON.FeatureCollection<GeoJSON.Point, TurbineProperties> & {
  storage_crs_srid: number;
  storage_crs_name: string | null;
};

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
    queryFn: () => apiRequest<TurbineFeatureCollection>(url),
    enabled,
  });
}
