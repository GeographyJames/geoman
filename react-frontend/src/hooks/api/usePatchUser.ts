import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

interface PatchUserRequest {
  team_id: number;
}

export function usePatchUser() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();
  const baseUrl = __URLS__.api.base + __URLS__.api.users;

  return useMutation<void, Error, { userId: number; patch: PatchUserRequest }>({
    mutationFn: async ({ userId, patch }) => {
      await apiRequest(`${baseUrl}/${userId}`, {
        method: "PATCH",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(patch),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["users"] });
    },
  });
}
