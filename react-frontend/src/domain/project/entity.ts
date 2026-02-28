

import type { ProjectOutputDTO } from "./outputDTO";
import { Status, type Visibility } from "../types";

export default class Project {
    outputDto: ProjectOutputDTO
    constructor(outputDto: ProjectOutputDTO ){
        this.outputDto = outputDto
    }
    get name(): string {
        return this.outputDto.properties.name
    }
    get id(): number {
        return this.outputDto.id
    }
    get searchAreaId():number | null{
        return this.outputDto.properties.search_area_id
    }
    get url(): string {
        return `/project/${this.searchAreaId ? this.id : this.slug}`
    }
    get slug(): string {
        return this.outputDto.properties.slug
    }
    get status(): Status {
        return this.outputDto.properties.status
    }
    get visibility(): Visibility {
        return this.outputDto.properties.visibility
    }
    get added(): Date {
        return new Date(this.outputDto.properties.added)
    }
    get centroid(): GeoJSON.Point | null {
        return this.outputDto.geometry
    }


    get ownerFirstName(): string {
        return this.outputDto.properties.owner_first_name
    }
    get ownerLastName(): string {
        return this.outputDto.properties.owner_last_name
    }
    get archived(): boolean {
        return this.outputDto.properties.status === Status.Archived
    }


    get latitude(): number| undefined {return this.outputDto.geometry?.coordinates[1]}
    get longitude(): number | undefined {return this.outputDto.geometry?.coordinates[0]}
    get crsSrid(): number | null { return this.outputDto.properties.crs_srid }
    get crsName(): string | null { return this.outputDto.properties.crs_name }
    get centroidX(): number | null { return this.outputDto.properties.centroid_x }
    get centroidY(): number | null { return this.outputDto.properties.centroid_y }


}