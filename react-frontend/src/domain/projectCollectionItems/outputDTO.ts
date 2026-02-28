import type { TeamOutputDto } from "../team/outputDto"
import type { Status } from "../types"

export interface ProjectCollectionItems extends GeoJSON.FeatureCollection{
    features: ProjectCollectionItem[]
}

export interface ProjectCollectionItem extends GeoJSON.Feature {
    id: number
    properties: {
        status: Status,
        name: string,
        added: string,
        added_by_first_name: string,
        added_by_last_name: string,
        added_by_id: number,
        added_by_team: TeamOutputDto,
        last_updated: string,
        last_updated_by_first_name: string,
        last_updated_by_last_name: string,
        last_updated_by_id: number,
        last_updated_by_team: TeamOutputDto,
        storage_crs_srid: number,
        is_primary: boolean,
        project_id: number,
        collection_id: number,
        collection_title: string,
        area_ellipsoidal_m2?: number


    }

}