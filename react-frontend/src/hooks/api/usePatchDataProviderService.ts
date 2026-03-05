import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { DataProviderServiceType } from "@/domain/data_provider/types";
import { CACHE_KEY_DATA_PROVIDER_SERVICES } from "@/cache_keys";

interface PatchDataProviderServiceRequest {
  name?: string;
  service_type?: DataProviderServiceType;
  base_url?: string | null;
  description?: string | null;
}

export function usePatchDataProviderService() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.data_provider_services;

  return useMutation<void, ApiError, { id: number; patch: PatchDataProviderServiceRequest }>({
    mutationFn: async ({ id, patch }) => {
      await apiRequest(`${baseUrl}/${id}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(patch),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_DATA_PROVIDER_SERVICES });
    },
  });
}
