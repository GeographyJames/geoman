import { CACHE_KEY_PROJECT_COLLECTION_ITEMS, CACHE_KEY_PROJECT_COLLECTIONS, CACHE_KEY_PROJECTS } from "@/cache_keys";
import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

interface PatchProjectFeatureDTO {
    status?: "ACTIVE" | "ARCHIVED" | "DELETED"
    primary?: boolean
    name?: string
}

export function usePatchProjectFeature() {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();

  return useMutation<void, Error, {projectId: number, collectionId: string, id: number; dto: PatchProjectFeatureDTO }>({
    mutationFn: async ({ projectId, collectionId, id, dto }) => {
      await apiRequest(`${__URLS__.api.base}${__URLS__.api.project_features}/${projectId}/${collectionId}/${id}`, {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(dto),
      });
    },
    onSuccess: (_, { projectId, collectionId }) => {
      queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECT_COLLECTION_ITEMS(projectId, collectionId) }),
      queryClient.invalidateQueries({queryKey: CACHE_KEY_PROJECT_COLLECTIONS(projectId)});
      queryClient.invalidateQueries({queryKey: CACHE_KEY_PROJECTS})
    },
  });
}