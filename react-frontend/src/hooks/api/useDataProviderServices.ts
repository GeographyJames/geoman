import { useQuery } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";
import type { DataProviderService } from "@/domain/data_provider/types";
import { CACHE_KEY_DATA_PROVIDER_SERVICES } from "@/cache_keys";

export function useDataProviderServices() {
  const apiRequest = useApiRequest();
  const url = __URLS__.api.base + __URLS__.api.data_provider_services;
  return useQuery({
    queryKey: CACHE_KEY_DATA_PROVIDER_SERVICES,
    queryFn: () => apiRequest<DataProviderService[]>(url).then((d) => d ?? []),
  });
}
