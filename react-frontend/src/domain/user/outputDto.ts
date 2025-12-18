import type { TeamOutputDto } from "../team/outputDto";

export interface UserOutputDto {
    id: number,
    first_name: string,
    last_name: string,
    team: TeamOutputDto
}