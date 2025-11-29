import { useQuery } from "@tanstack/react-query";
import { useAuth } from "@clerk/clerk-react";

export interface ApiKey {
  id: number;
  name: string;
  created: string;
  last_used: string | null;
  expiry: string;
}

export function useApiKeys() {
  const { getToken } = useAuth();

  return useQuery({
    queryKey: ["apiKeys"],
    queryFn: async (): Promise<ApiKey[]> => {
      const token = await getToken();

      const response = await fetch("/api/keys", {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch API keys: ${response.statusText}`);
      }

      return response.json();
    },
  });
}
