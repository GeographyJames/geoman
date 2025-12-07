import { useQuery } from "@tanstack/react-query";
import { useAuth } from "@clerk/clerk-react";
import { User } from "./useCurrentUser";

export function useUsers() {
  const { getToken } = useAuth();

  return useQuery({
    queryKey: ["users"],
    queryFn: async (): Promise<User[]> => {
      const token = await getToken();

      const response = await fetch("/api/users", {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch users: ${response.statusText}`);
      }

      return response.json();
    },
  });
}
