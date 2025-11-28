import { useMutation } from "@tanstack/react-query";
import { useAuth } from "@clerk/clerk-react";

interface CreateApiKeyRequest {
  key_name: string;
}

interface CreateApiKeyResponse {
  api_key: string;
}

export function useCreateApiKey() {
  const { getToken } = useAuth();

  return useMutation({
    mutationFn: async (request: CreateApiKeyRequest): Promise<CreateApiKeyResponse> => {
      const token = await getToken();

      const response = await fetch("/api/keys", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${token}`,
        },
        body: JSON.stringify(request),
      });

      if (!response.ok) {
        throw new Error(`Failed to create API key: ${response.statusText}`);
      }

      return response.json();
    },
  });
}
