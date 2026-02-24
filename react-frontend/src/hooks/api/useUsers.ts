import { useQuery } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";
import type { UserOutputDto } from "@/domain/user/outputDto";
import User from "@/domain/user/entity";

export function useUsers() {
  const apiRequest = useApiRequest();
  const url = __URLS__.api.base + __URLS__.api.users;

  return useQuery({
    queryKey: ["users"],
    queryFn: async (): Promise<User[]> => {
      const data = await apiRequest<UserOutputDto[]>(url);
      return (data ?? []).map((dto) => new User(dto));
    },
  });
}
