import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { LayerCategory } from "@/domain/data_provider/types";
import { CACHE_KEY_DATA_PROVIDER_LAYERS } from "@/cache_keys";

interface PatchDataProviderLayerRequest {
  name?: string;
  abbreviation?: string | null;
  source?: unknown;
  category?: LayerCategory;
  description?: string | null;
  enabled?: boolean;
  style_config?: unknown;
  display_options?: unknown;
  country_code?: string | null;
  subdivision?: string | null;
  sort_order?: number;
}

export function usePatchDataProviderLayer() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.data_provider_layers;

  return useMutation<void, ApiError, { id: number; patch: PatchDataProviderLayerRequest }>({
    mutationFn: async ({ id, patch }) => {
      await apiRequest(`${baseUrl}/${id}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(patch),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDER_LAYERS });
    },
  });
}
