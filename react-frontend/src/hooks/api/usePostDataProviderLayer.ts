import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { LayerCategory } from "@/domain/data_provider/types";
import { CACHE_KEY_DATA_PROVIDER_LAYERS } from "@/cache_keys";

interface PostDataProviderLayerRequest {
  service_id: number;
  name: string;
  abbreviation?: string | null;
  source: unknown;
  category?: LayerCategory;
  description?: string | null;
  enabled_geoman?: boolean;
  enabled_figure_tool?: boolean;
  style_config?: unknown;
  display_options?: unknown;
  country_code?: string | null;
  subdivision?: string | null;
  sort_order?: number;
}

export function usePostDataProviderLayer() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const url = __URLS__.api.base + __URLS__.api.data_provider_layers;

  return useMutation<number, ApiError, PostDataProviderLayerRequest>({
    mutationFn: async (payload) => {
      const id = await apiRequest<number>(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
      return id as number;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDER_LAYERS });
    },
  });
}
