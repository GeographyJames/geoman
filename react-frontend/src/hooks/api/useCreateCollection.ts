import { CACHE_KEY_COLLECTIONS } from "@/cache_keys"
import { useApiRequest } from "@/lib/api"
import { useMutation, useQueryClient } from "@tanstack/react-query"

interface CreateCollectionRequest {
    title: string
    geometry_type: string
    description?: string
    project_id?: number
}

export const useCreateCollection = () => {
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
            queryClient.invalidateQueries({ queryKey: CACHE_KEY_COLLECTIONS })
            queryClient.invalidateQueries({ queryKey: ["project-collections"] })
        },
    })
}
