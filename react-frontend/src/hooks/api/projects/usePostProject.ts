import { CACHE_KEY_PROJECTS } from "@/cache_keys";
import type ProjectInputDTO from "@/domain/project/inputDTO";
import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export const usePostProject = () => {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();

  return useMutation<number, Error, ProjectInputDTO>({
    mutationFn: async (dto) => {
      const data = await apiRequest<{ id: number }>("/api/projects", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(dto),
      });
      return data!.id;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECTS });
    },
  });
};
