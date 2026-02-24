import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export function useDeleteBusinessUnit() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.business_units;

  return useMutation<void, Error, number>({
    mutationFn: async (id) => {
      await apiRequest(`${baseUrl}/${id}`, { method: "DELETE" });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["business_units"] });
    },
  });
}
