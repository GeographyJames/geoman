import { useQuery } from "@tanstack/react-query";
import { useAuth } from "@clerk/clerk-react";

export interface Team {
  id: number;
  name: string;
}

export interface User {
  id: number;
  first_name: string;
  last_name: string;
  clerk_id: string | null;
  team: Team | null;
}

export function useCurrentUser() {
  const { getToken } = useAuth();

  return useQuery({
    queryKey: ["currentUser"],
    queryFn: async (): Promise<User> => {
      const token = await getToken();

      const response = await fetch("/api/users/current", {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch current user: ${response.statusText}`);
      }

      return response.json();
    },
  });
}
