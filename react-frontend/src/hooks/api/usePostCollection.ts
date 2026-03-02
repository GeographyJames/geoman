import { CACHE_KEY_COLLECTION_LIST_ITEMS,  CACHE_KEY_PROJECT_COLLECTIONS_ALL } from "@/cache_keys"
import { useApiRequest } from "@/lib/api"
import { useMutation, useQueryClient } from "@tanstack/react-query"

interface CreateCollectionRequest {
    title: string
    geometry_type: string
    description?: string
    project_id?: number
}

export const usePostCollection = () => {
    const apiRequest = useApiRequest()
    const queryClient = useQueryClient()

    return useMutation<void, Error, CreateCollectionRequest>({
        mutationFn: async (dto) => {
            await apiRequest("/api/collections", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(dto),
            })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_COLLECTION_LIST_ITEMS })
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECT_COLLECTIONS_ALL})
        },
    })
}
