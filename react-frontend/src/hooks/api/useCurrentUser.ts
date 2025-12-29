import { useQuery } from "@tanstack/react-query";
import type { UserOutputDto } from "@/domain/user/outputDto";
import { useApiRequest } from "@/lib/api";
import { useAuth } from "@clerk/clerk-react";
import User from "@/domain/user/entity";

export interface Team {
  id: number;
  name: string;
}



export function useCurrentUser() {
  const apiRequest = useApiRequest();
  const { isLoaded } = useAuth();
  const url = __URLS__.api.base + __URLS__.api.users + "/current";

  return useQuery({
    queryKey: ["currentUser"],
    queryFn: async () => {
      const data = await apiRequest<UserOutputDto>(url);
      if (data) {
        return new User(data);
      }
    },
    enabled: isLoaded, // Wait for Clerk to initialize
  });
}
