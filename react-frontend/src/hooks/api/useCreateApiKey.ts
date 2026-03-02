import { useMutation } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";

interface CreateApiKeyRequest {
  key_name: string;
}

interface CreateApiKeyResponse {
  api_key: string;
  id: number;
}

export function useCreateApiKey() {
  const apiRequest = useApiRequest();

  return useMutation({
    mutationFn: async (request: CreateApiKeyRequest) => {
      const result = await apiRequest<CreateApiKeyResponse>(__URLS__.api.base + __URLS__.api.keys, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(request),
      });
      return result!;
    },
  });
}
