import type { TeamOutputDto } from "../team/outputDto";

export type Status = "ACTIVE" | "ARCHIVED" | "DELETED";
export type Visibility = "PRIVATE" | "TEAM" | "PUBLIC";



export interface ProjectProperties {
    name: string;
    added: string;
    // Owner fields (flattened)
    owner_id: number;
    owner_first_name: string;
    owner_last_name: string;
    owner_team: TeamOutputDto;
    // AddedBy fields (flattened)
    added_by_id: number;
    added_by_first_name: string;
    added_by_last_name: string;
    added_by_team: TeamOutputDto;
    // Core properties
    technologies: string[];
    country_code: string; // ISO 3166-1-ALPHA-2
    subdivisions: string[];
    status: Status;
    visibility: Visibility;
    crs_srid: number | null;
    // LastUpdatedBy fields (flattened)
    last_updated_by_id: number;
    last_updated_by_first_name: string;
    last_updated_by_last_name: string;
    last_updated_by_team: TeamOutputDto;
    last_updated: string;
    slug: string;
    search_area_id: number | null;
    search_site_name: string | null;
}