import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { CACHE_KEY_DATA_PROVIDER_LAYERS } from "@/cache_keys";

export function useDeleteDataProviderLayer() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.data_provider_layers;

  return useMutation<void, Error, number>({
    mutationFn: async (id) => {
      await apiRequest(`${baseUrl}/${id}`, { method: "DELETE" });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDER_LAYERS });
    },
  });
}
