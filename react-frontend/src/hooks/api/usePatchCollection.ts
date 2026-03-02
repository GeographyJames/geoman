import { CACHE_KEY_COLLECTION_LIST_ITEMS,  CACHE_KEY_PROJECT_COLLECTIONS_ALL } from "@/cache_keys"
import { useApiRequest } from "@/lib/api"
import { useMutation, useQueryClient } from "@tanstack/react-query"

interface PatchCollectionRequest {
    title?: string
    description?: string | null
    status?: "ACTIVE" | "ARCHIVED" | "DELETED"
}

export const usePatchCollection = () => {
    const apiRequest = useApiRequest()
    const queryClient = useQueryClient()

    return useMutation<void, Error, { id: number; patch: PatchCollectionRequest }>({
        mutationFn: async ({ id, patch }) => {
            await apiRequest(`/api/collections/${id}`, {
                method: "PATCH",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify(patch),
            })
        },
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_COLLECTION_LIST_ITEMS })
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_PROJECT_COLLECTIONS_ALL })
        },
    })
}
