import { useAuth } from "@clerk/clerk-react";

export class ApiError extends Error {
  status: number;
  long_message: string
  constructor(message: string, status: number, long_message: string) {
    super(message);
    this.status = status;
    this.long_message = long_message
  }
}

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
      let message = `${response.statusText} (${response.status})`;
      const contentType = response.headers.get("content-type");

      let long_message = "";
      if (contentType?.includes("application/json")) {
        try {
          const errorJson = await response.json();
          if (typeof errorJson?.message === "string") {
            message = errorJson.message;
          }
          if (typeof errorJson?.long_message === "string") {
            long_message = errorJson.long_message;
          }
        } catch {
          // ignore
        }
      } else {
        try {
          const text = await response.text();
          if (text) message = text;
        } catch {
          // ignore
        }
      }

      throw new ApiError(message, response.status, long_message);
    }
    if (response.status === 204) {
      return undefined
    }
    const contentType = response.headers.get("content-type");
    if (contentType?.includes("application/json") || contentType?.includes("application/geo+json")) {
      return response.json();
    }

    return undefined;
  };
}