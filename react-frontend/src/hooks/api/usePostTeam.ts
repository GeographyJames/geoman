import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

interface PostTeamRequest {
  name: string;
  business_unit?: number | null;
}

export function usePostTeam() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const url = __URLS__.api.base + __URLS__.api.teams;

  return useMutation<number, ApiError, PostTeamRequest>({
    mutationFn: async (payload) => {
      const id = await apiRequest<number>(url, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
      });
      return id as number;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["teams"] });
    },
  });
}
