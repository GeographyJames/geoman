import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

interface PatchBusinessUnitRequest {
  name?: string;
}

export function usePatchBusinessUnit() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.business_units;

  return useMutation<void, ApiError, { id: number; patch: PatchBusinessUnitRequest }>({
    mutationFn: async ({ id, patch }) => {
      await apiRequest(`${baseUrl}/${id}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(patch),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["business_units"] });
    },
  });
}
