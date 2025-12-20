import { CACHE_KEY_PROJECTS } from "@/cache_keys";
import type ProjectInputDTO from "@/domain/project/inputDTO";


import { useAuth } from "@clerk/clerk-react";
import { useMutation, useQueryClient } from "@tanstack/react-query"

export const usePostProject = () => {
    const { getToken } = useAuth();
    const queryClient = useQueryClient()
    return useMutation<number, Error, ProjectInputDTO>({mutationFn: async (dto: ProjectInputDTO): Promise<number> => {
        const token = await getToken();
      const response = await fetch("/api/projects", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify(dto),
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(errorText || "Failed to create project");
      }

      // assume the API returns JSON like { id: number }
      const data = await response.json();
      return data.id;
    },

    // on success: invalidate or update relevant queries
    onSuccess: () => {
      // invalidate the project list query so it refetches
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECTS });
        
    }})
}