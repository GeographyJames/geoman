import type { TeamOutputDto } from "./outputDto"

export default class Team {
    outputDto: TeamOutputDto

    constructor(outputDto: TeamOutputDto) {
        this.outputDto = outputDto
    }

    get id(): number { return this.outputDto.id }
    get name(): string { return this.outputDto.name }
    get businessUnitId(): number | null { return this.outputDto.business_unit_id }
}