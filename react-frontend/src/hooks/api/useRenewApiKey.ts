import { useMutation } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";

export function useRenewApiKey() {
  const apiRequest = useApiRequest();

  return useMutation({
    mutationFn: (keyId: number) =>
      apiRequest(`${__URLS__.api.base}${__URLS__.api.keys}/${keyId}/renew`, { method: "PATCH" }),
  });
}
