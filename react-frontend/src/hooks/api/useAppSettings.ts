import type { TechnologyOutputDto } from "@/domain/technology/outputDto";
import { useAuth } from "@clerk/clerk-react";
import { useQuery } from "@tanstack/react-query";

export interface AppSettings {
    technologies: TechnologyOutputDto[]
}

export function useAppSettings() {
  const { getToken } = useAuth();

  return useQuery({
    queryKey: ["appSettings"],
    queryFn: async (): Promise<AppSettings> => {
      const token = await getToken();

      const response = await fetch(`${__URLS__.api.base}${ __URLS__.api.app_settings}`, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error(`Failed to fetch app settings: ${response.statusText}`);
      }

      return response.json();
    },
  });
}