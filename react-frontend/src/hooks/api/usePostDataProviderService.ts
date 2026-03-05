import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { DataProviderServiceType } from "@/domain/data_provider/types";
import { CACHE_KEY_DATA_PROVIDER_SERVICES } from "@/cache_keys";

interface PostDataProviderServiceRequest {
  provider_id: number;
  name: string;
  service_type: DataProviderServiceType;
  base_url?: string | null;
  description?: string | null;
}

export function usePostDataProviderService() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const url = __URLS__.api.base + __URLS__.api.data_provider_services;

  return useMutation<number, ApiError, PostDataProviderServiceRequest>({
    mutationFn: async (payload) => {
      const id = await apiRequest<number>(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
      return id as number;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDER_SERVICES });
    },
  });
}
