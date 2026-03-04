import { useQuery } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";
import type { DataProvider } from "@/domain/data_provider/types";
import { CACHE_KEY_DATA_PROVIDERS } from "@/cache_keys";

export function useDataProviders() {
  const apiRequest = useApiRequest();
  const url = __URLS__.api.base + __URLS__.api.data_providers;
  return useQuery({
    queryKey: CACHE_KEY_DATA_PROVIDERS,
    queryFn: () => apiRequest<DataProvider[]>(url).then((d) => d ?? []),
  });
}
