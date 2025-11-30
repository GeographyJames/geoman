import { useMutation } from "@tanstack/react-query";
import { useAuth } from "@clerk/clerk-react";

export function useRevokeApiKey() {
  const { getToken } = useAuth();

  return useMutation({
    mutationFn: async (keyId: number): Promise<void> => {
      const token = await getToken();

      const response = await fetch(`/api/keys/${keyId}/revoke`, {
        method: "PATCH",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to revoke API key: ${response.statusText}`);
      }
    },
  });
}
