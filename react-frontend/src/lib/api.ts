import { useAuth } from "@clerk/clerk-react";

/**
 * Simple hook that returns an API request function.
 * If the user is signed in, automatically includes the auth token.
 * The backend determines whether auth is required for each endpoint.
 *
 * Usage in TanStack Query hooks:
 *
 * const apiRequest = useApiRequest();
 *
 * useQuery({
 *   queryKey: ['projects'],
 *   queryFn: () => apiRequest<GeoJSON.FeatureCollection>('/ogcapi/collections/projects/items'),
 * });
 */
export function useApiRequest() {
  const { getToken, isSignedIn } = useAuth();

  return async <T>(url: string, options?: RequestInit): Promise<T | undefined> => {
    // Get token if user is signed in
    let token: string | null = null;
    if (isSignedIn) {
      token = await getToken();
    }

    const response = await fetch(url, {
      ...options,
      headers: {
        ...(token && { Authorization: `Bearer ${token}` }),
        ...options?.headers,
      },
    });

    if (!response.ok) {
      throw new Error(
        `API request failed: ${response.statusText} (${response.status})`
      );
    }
    if (response.status === 204) {
      return undefined
    }
    const contentType = response.headers.get("content-type");
    if (contentType?.includes("application/json")) {
      return response.json();
    }

    return undefined;
  };
}