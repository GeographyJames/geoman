export const CACHE_KEY_PROJECTS = ["projects"]
export const CACHE_KEY_PROJECT_COLLECTIONS= (id: number) => ["project-collections", id]
export const CACHE_KEY_PROJECT_COLLECTION_ITEMS = (projectId: number, collectionId: string) => ["project-collection-items", projectId, collectionId]