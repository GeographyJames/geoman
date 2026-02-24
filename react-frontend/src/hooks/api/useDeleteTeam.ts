import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export function useDeleteTeam() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.teams;

  return useMutation<void, Error, number>({
    mutationFn: async (teamId) => {
      await apiRequest(`${baseUrl}/${teamId}`, { method: "DELETE" });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["teams"] });
    },
  });
}
