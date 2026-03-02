const OGC_PROJECT_COLLECTIONS_KEY = "ogc-project-collections"

export const CACHE_KEY_PROJECTS = ["projects"]
export const CACHE_KEY_COLLECTION_LIST_ITEMS= ["collections"]
export const CACHE_KEY_PROJECT_COLLECTIONS= (id: number) => [OGC_PROJECT_COLLECTIONS_KEY, id]
export const CACHE_KEY_PROJECT_COLLECTIONS_ALL = [OGC_PROJECT_COLLECTIONS_KEY]
export const CACHE_KEY_PROJECT_COLLECTION_ITEMS = (projectId: number, collectionId: string) => ["project-collection-items", projectId, collectionId]