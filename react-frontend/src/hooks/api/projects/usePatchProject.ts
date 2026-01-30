import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";
import { CACHE_KEY_PROJECTS } from "@/cache_keys";

interface PatchProjectDTO {
  status?: "ACTIVE" | "ARCHIVED" | "DELETED";
}

export function usePatchProject() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();

  return useMutation<void, Error, { id: number; dto: PatchProjectDTO }>({
    mutationFn: async ({ id, dto }) => {
      await apiRequest(`/api/projects/${id}`, {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(dto),
      });
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECTS });
    },
  });
}
