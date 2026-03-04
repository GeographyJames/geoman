import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { CACHE_KEY_DATA_PROVIDERS } from "@/cache_keys";

interface PatchDataProviderRequest {
  name?: string;
  description?: string | null;
  country_code?: string | null;
  subdivision?: string | null;
}

export function usePatchDataProvider() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.data_providers;

  return useMutation<void, ApiError, { id: number; patch: PatchDataProviderRequest }>({
    mutationFn: async ({ id, patch }) => {
      await apiRequest(`${baseUrl}/${id}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(patch),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDERS });
    },
  });
}
