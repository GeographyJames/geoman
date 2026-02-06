import { CACHE_KEY_COLLECTIONS } from "@/cache_keys"
import { useApiRequest } from "@/lib/api"
import { useQuery } from "@tanstack/react-query"

export interface Collection {
    id: number
    title: string
    geometry_type: string
    description: string | null
    active_feature_count: number
    archived_feature_count: number
}

export function useCollections() {
    const apiRequest = useApiRequest()
    return useQuery({
        queryKey: CACHE_KEY_COLLECTIONS,
        queryFn: () => apiRequest<Collection[]>("/api/collections")
    })
}
