import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

interface PostBusinessUnitRequest {
  name: string;
}

export function usePostBusinessUnit() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const url = __URLS__.api.base + __URLS__.api.business_units;

  return useMutation<number, ApiError, PostBusinessUnitRequest>({
    mutationFn: async (payload) => {
      const id = await apiRequest<number>(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
      return id as number;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["business_units"] });
    },
  });
}
