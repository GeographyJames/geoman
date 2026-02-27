import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { CACHE_KEY_PROJECT_COLLECTION_ITEMS, CACHE_KEY_PROJECT_COLLECTIONS, CACHE_KEY_PROJECTS } from "@/cache_keys";
import type { Shapefile } from "@/lib/shapefile";


export interface TurbineLayoutParams {
  primary?: boolean;
  hubHeightDefaultMetre?: number;
  rotorDiameterDefaultMetre?: number;
  turbineNumberField?: string;
  rotorDiameterField?: string;
  hubHeightField?: string;
}

interface PostProjectFeatureParams {
  projectId: number;
  collectionId: number;
  name: string;
  shapefile: Shapefile;
  turbineLayout?: TurbineLayoutParams;
}

export const usePostProjectFeature = () => {
  const apiRequest = useApiRequest();
  const queryClient = useQueryClient();

  return useMutation<number, Error, PostProjectFeatureParams>({
    mutationFn: async ({ projectId, collectionId, name, shapefile, turbineLayout }) => {
      const form = new FormData();
      shapefile.addToForm(form);
      form.append("name", name);
      if (turbineLayout) {
        if (turbineLayout.primary != null) form.append("primary", String(turbineLayout.primary));
        if (turbineLayout.hubHeightDefaultMetre != null) form.append("hub_height_default_metre", String(turbineLayout.hubHeightDefaultMetre));
        if (turbineLayout.rotorDiameterDefaultMetre != null) form.append("rotor_diameter_default_metre", String(turbineLayout.rotorDiameterDefaultMetre));
        if (turbineLayout.turbineNumberField) form.append("turbine_number_field", turbineLayout.turbineNumberField);
        if (turbineLayout.rotorDiameterField) form.append("rotor_diameter_field", turbineLayout.rotorDiameterField);
        if (turbineLayout.hubHeightField) form.append("hub_height_field", turbineLayout.hubHeightField);
      }

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
      queryClient.invalidateQueries({queryKey: CACHE_KEY_PROJECTS});
    },
  });
};
