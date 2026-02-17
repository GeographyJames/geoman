import { CACHE_KEY_PROJECT_COLLECTIONS } from "@/cache_keys"
import { Status } from "@/domain/types"
import { useApiRequest } from "@/lib/api"
import { useQuery } from "@tanstack/react-query"

export interface CollectionsResponse {
    collections: Collection[]
}

export interface Collection {
     title: string
     id: string
     geometry_type: string

}

export function useProjectCollections({projectId}: {projectId: number}) {
    const apiRequest = useApiRequest()
    const url = __URLS__.ogc_api.base + __URLS__.ogc_api.project + "/" +projectId + __URLS__.ogc_api.collections+`?status=${Status.Active},${Status.Archived}`
    return useQuery({
        queryKey: CACHE_KEY_PROJECT_COLLECTIONS(projectId),
        queryFn:()=>  apiRequest<CollectionsResponse>(url)
    })
}