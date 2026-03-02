import { useMutation } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";

export function useRevokeApiKey() {
  const apiRequest = useApiRequest();

  return useMutation({
    mutationFn: (keyId: number) =>
      apiRequest(`/api/keys/${keyId}/revoke`, { method: "PATCH" }),
  });
}
