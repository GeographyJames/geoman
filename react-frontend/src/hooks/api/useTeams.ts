import { useQuery } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";
import type { TeamOutputDto } from "@/domain/team/outputDto";
import Team from "@/domain/team/entity";

export function useTeams() {
  const apiRequest = useApiRequest();
  const url = __URLS__.api.base + __URLS__.api.teams;

  return useQuery({
    queryKey: ["teams"],
    queryFn: async (): Promise<Team[]> => {
      
      const data = await apiRequest<TeamOutputDto[]>(url);
      console.log(`raw data ${data}`)
      return (data ?? []).map((dto) => new Team(dto));
    },
  });
}
