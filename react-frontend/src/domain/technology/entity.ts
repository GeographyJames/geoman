import type { TechnologyOutputDto } from "./outputDto";

export default class Technology{
    outputDto: TechnologyOutputDto
    constructor(outputDto: TechnologyOutputDto) {
        this.outputDto = outputDto
    }
}