import Team from "../team/entity"
import type { UserOutputDto } from "./outputDto"

export default class User {
    outputDto: UserOutputDto
    constructor(outputDto: UserOutputDto) {
        this.outputDto = outputDto
    }
    get firstName(): string {
        return this.outputDto.first_name
    }
    get lastName(): string {
        return this.outputDto.last_name
    }
    get operatingCountryId(): string {
        return this.outputDto.operating_country_code
    }
    get teamId(): number {
        return this.outputDto.team.id
    }
    get team(): Team {
        return new Team({id: this.outputDto.team.id, name: this.outputDto.team.name})
    }
    get id(): number {
        return this.outputDto.id
    }
    get operatingCountryCode(): string | undefined {return this.outputDto.operating_country_code}
    get isAdmin(): boolean {
        return this.outputDto.admin
    }
}