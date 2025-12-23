import type { TeamOutputDto } from "./outputDto"

export default class Team {
    outputDto: TeamOutputDto

    constructor(outputDto: TeamOutputDto) {
        this.outputDto = outputDto
    }
get name(): string {return this.outputDto.name}
}