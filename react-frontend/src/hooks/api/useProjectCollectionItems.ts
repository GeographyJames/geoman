
import { CACHE_KEY_PROJECT_COLLECTION_ITEMS } from "@/cache_keys";
import type { ProjectCollectionItems } from "@/domain/projectCollectionItems/outputDTO";
import { Status } from "@/domain/types";
import { useApiRequest } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";

export function useProjectCollectionItems({projectId, collectionId, enabled = true}: {projectId: number, collectionId: string, enabled?: boolean}){
    const apiRequest = useApiRequest()
    const url = `${__URLS__.ogc_api.base}${__URLS__.ogc_api.project}/${projectId}${__URLS__.ogc_api.collections}/${collectionId}/items?status=${Status.Active},${Status.Archived}`
    return useQuery({
        queryKey: CACHE_KEY_PROJECT_COLLECTION_ITEMS(projectId, collectionId),
        queryFn: ()=> apiRequest<ProjectCollectionItems>(url),
        enabled,
        retry: false,
    })
}