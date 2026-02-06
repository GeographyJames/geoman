import { CACHE_KEY_COLLECTIONS } from "@/cache_keys"
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
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_COLLECTIONS })
        },
    })
}
