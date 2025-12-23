import type Team from "../team/entity";
import type { TeamOutputDto } from "../team/outputDto";

export interface UserOutputDto {
  id: number;
  first_name: string;
  last_name: string;
  clerk_id: string | null;
  team: TeamOutputDto;
  operating_country_code: string
}