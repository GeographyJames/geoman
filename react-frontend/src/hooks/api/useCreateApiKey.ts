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
    mutationFn: (request: CreateApiKeyRequest) =>
      apiRequest<CreateApiKeyResponse>("/api/keys", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(request),
      }),
  });
}
