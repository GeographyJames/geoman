import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  CACHE_KEY_DATA_PROVIDERS,
  CACHE_KEY_DATA_PROVIDER_SERVICES,
  CACHE_KEY_DATA_PROVIDER_LAYERS,
} from "@/cache_keys";

export function useDeleteDataProvider() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.data_providers;

  return useMutation<void, Error, number>({
    mutationFn: async (id) => {
      await apiRequest(`${baseUrl}/${id}`, { method: "DELETE" });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDERS });
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDER_SERVICES });
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDER_LAYERS });
    },
  });
}
