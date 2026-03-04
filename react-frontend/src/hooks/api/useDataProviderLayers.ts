import { useQuery } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";
import type { DataProviderLayer } from "@/domain/data_provider/types";
import { CACHE_KEY_DATA_PROVIDER_LAYERS } from "@/cache_keys";

export function useDataProviderLayers() {
  const apiRequest = useApiRequest();
  const url = __URLS__.api.base + __URLS__.api.data_provider_layers;
  return useQuery({
    queryKey: CACHE_KEY_DATA_PROVIDER_LAYERS,
    queryFn: () => apiRequest<DataProviderLayer[]>(url).then((d) => d ?? []),
  });
}
