import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { CACHE_KEY_PROJECT_COLLECTION_ITEMS, CACHE_KEY_PROJECT_COLLECTIONS } from "@/cache_keys";
import type { Shapefile } from "@/lib/shapefile";


interface PostProjectFeatureParams {
  projectId: number;
  collectionId: number;
  name: string;
  shapefile: Shapefile;
}

export const usePostProjectFeature = () => {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();

  return useMutation<number, Error, PostProjectFeatureParams>({
    mutationFn: async ({ projectId, collectionId, name, shapefile }) => {
      const form = new FormData();
      shapefile.addToForm(form);
      form.append("name", name);

      const data = await apiRequest<{ id: number }>(
        `/api/features/${projectId}/${collectionId}`,
        {
          method: "POST",
          body: form,
        },
      );
      return data!.id;
    },
    onSuccess: (_, { projectId, collectionId }) => {
      queryClient.invalidateQueries({
        queryKey: CACHE_KEY_PROJECT_COLLECTION_ITEMS(
          projectId,
          String(collectionId),
        ),
        
      })
      queryClient.invalidateQueries({queryKey: CACHE_KEY_PROJECT_COLLECTIONS(projectId)});
    },
  });
};
