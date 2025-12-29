import type { TechnologyOutputDto } from "@/domain/technology/outputDTO";
import { useApiRequest } from "@/lib/api";
import { useAuth } from "@clerk/clerk-react";
import { useQuery } from "@tanstack/react-query";


export interface AppSettingsResponse {
    technologies: TechnologyOutputDto[]
}

export function useAppSettings() {
    const apiRequest = useApiRequest();
    const { isLoaded } = useAuth();
    const url = __URLS__.api.base + __URLS__.api.app_settings;


  return useQuery({
    queryKey: ["appSettings"],
    queryFn: async () => {
      const data = await apiRequest<AppSettingsResponse>(url);
      return data
    },
    enabled: isLoaded, // Wait for Clerk to initialize
  });
}