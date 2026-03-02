import { useQuery } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";

export interface ApiKey {
  id: number;
  name: string;
  created: string;
  last_used: string | null;
  expiry: string;
  last_used_ip: string | null;
  last_used_user_agent: string | null;
}

export function useApiKeys() {
  const apiRequest = useApiRequest();

  return useQuery({
    queryKey: ["apiKeys"],
    queryFn: () => apiRequest<ApiKey[]>("/api/keys").then(d => d ?? []),
  });
}
