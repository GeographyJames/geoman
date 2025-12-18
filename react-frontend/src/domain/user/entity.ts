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

}