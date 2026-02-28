import { CACHE_KEY_PROJECT_COLLECTION_ITEMS, CACHE_KEY_PROJECT_COLLECTIONS, CACHE_KEY_PROJECTS } from "@/cache_keys";
import { useApiRequest } from "@/lib/api";
import { useMutation, useQueryClient } from "@tanstack/react-query";

interface DuplicateTurbineLayoutDTO {
    name?: string;
    hub_height_metre?: number;
    rotor_diameter_metre?: number;
    primary?: boolean;
}

export function useDuplicateTurbineLayout() {
    const apiRequest = useApiRequest();
    const queryClient = useQueryClient();

    return useMutation<number, Error, { projectId: number; collectionId: number; id: number; dto: DuplicateTurbineLayoutDTO }>({
        mutationFn: async ({ projectId, collectionId, id, dto }) => {
            const data = await apiRequest<number>(
                `/api/features/${projectId}/${collectionId}/${id}/duplicate`,
                {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify(dto),
                },
            );
            return data!;
        },
        onSuccess: (_, { projectId, collectionId }) => {
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECT_COLLECTION_ITEMS(projectId, String(collectionId)) });
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECT_COLLECTIONS(projectId) });
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECTS });
        },
    });
}
