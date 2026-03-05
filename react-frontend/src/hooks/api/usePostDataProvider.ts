import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { CACHE_KEY_DATA_PROVIDERS } from "@/cache_keys";

interface PostDataProviderRequest {
  name: string;
  country_code?: string | null;
  subdivision?: string | null;
}

export function usePostDataProvider() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const url = __URLS__.api.base + __URLS__.api.data_providers;

  return useMutation<number, ApiError, PostDataProviderRequest>({
    mutationFn: async (payload) => {
      const id = await apiRequest<number>(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
      return id as number;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDERS });
    },
  });
}
