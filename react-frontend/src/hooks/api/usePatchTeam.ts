import { useApiRequest, ApiError } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

interface PatchTeamRequest {
  name?: string;
  business_unit?: number | null;
}

export function usePatchTeam() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.teams;

  return useMutation<void, ApiError, { teamId: number; patch: PatchTeamRequest }>({
    mutationFn: async ({ teamId, patch }) => {
      await apiRequest(`${baseUrl}/${teamId}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(patch),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["teams"] });
    },
  });
}
