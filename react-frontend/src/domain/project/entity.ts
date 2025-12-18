
import User from "../user/entity";
import type { UserOutputDto } from "../user/outputDto";
import type { ProjectOutputDto } from "./outputDTO";

export default class Project {
    outputDto: ProjectOutputDto
    constructor(outputDto: ProjectOutputDto ){
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
    get status(): string {
        return this.outputDto.properties.status
    }
    get added(): Date {
        return new Date(this.outputDto.properties.added)
    }
    get centroid(): GeoJSON.Point | null {
        return this.outputDto.geometry
    }
    get hasWind(): boolean {
        return true
    }
    get primaryLayoutTurbineCount(): number | null {
        return 10
    }
    get private(): boolean {
        return true
    }
get addedBy(): User {
    let dto: UserOutputDto = {
        first_name: this.outputDto.properties.added_by_first_name,
        last_name: this.outputDto.properties.added_by_last_name,
        id: this.outputDto.properties.added_by_id,
        team: this.outputDto.properties.added_by_team
    }
    return new User(dto)
}
}