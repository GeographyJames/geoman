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
    added: string
    added_by_id: number
    added_by_first_name: string
    added_by_last_name: string
    added_by_team: {
        id: number
        name: string
    }
}

export function useCollections() {
    const apiRequest = useApiRequest()
    return useQuery({
        queryKey: CACHE_KEY_COLLECTIONS,
        queryFn: () => apiRequest<Collection[]>("/api/collections")
    })
}
