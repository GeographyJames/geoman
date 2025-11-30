import { useMutation } from "@tanstack/react-query";
import { useAuth } from "@clerk/clerk-react";

export function useRenewApiKey() {
  const { getToken } = useAuth();

  return useMutation({
    mutationFn: async (keyId: number): Promise<void> => {
      const token = await getToken();

      const response = await fetch(`/api/keys/${keyId}/renew`, {
        method: "PATCH",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to renew API key: ${response.statusText}`);
      }
    },
  });
}
