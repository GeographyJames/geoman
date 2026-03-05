import { useQuery } from "@tanstack/react-query";

export interface ArcGISLayerInfo {
  id: number;
  name: string;
}

export interface ArcGISServicesDirectory {services: ArcGISService[]}

export interface ArcGISService {
    name: string, type: string, url: string
}

interface ArcGISDirectoryResponse {
  services?: ArcGISService[];
  error?: { message: string };
}

interface ArcGISFeatureServerResponse {
  layers?: ArcGISLayerInfo[];
  error?: { message: string };
}

/** Hits the ArcGIS services directory and returns the list of available services. */
export function useArcGISServiceInfo(baseUrl: string) {
  return useQuery({
    queryKey: ["arcgis-service-info", baseUrl],
    queryFn: async () => {
      const res = await fetch(`${baseUrl}?f=json`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = (await res.json()) as ArcGISDirectoryResponse;
      if (data.error) throw new Error(data.error.message);
      return data.services ?? [];
    },
    enabled: baseUrl.length > 0,
    retry: false,
    staleTime: 60_000,
  });
}

/** Hits a specific ArcGIS FeatureServer URL and returns its layer list. */
export function useArcGISFeatureServerLayers(featureServerUrl: string) {
  return useQuery({
    queryKey: ["arcgis-feature-server-layers", featureServerUrl],
    queryFn: async () => {
      const res = await fetch(`${featureServerUrl}?f=json`);
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const data = (await res.json()) as ArcGISFeatureServerResponse;
      if (data.error) throw new Error(data.error.message);
      return data.layers ?? [];
    },
    enabled: featureServerUrl.length > 0,
    retry: false,
    staleTime: 60_000,
  });
}
