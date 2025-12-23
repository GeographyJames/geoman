import { useQuery } from "@tanstack/react-query";
import { useApiRequest } from "@/lib/api";
import type { ProjectOutputDTO } from "@/domain/project/outputDTO";
import type { FeatureCollection, Point } from "geojson";
import type { ProjectProperties } from "@/domain/project/properties";
import { CACHE_KEY_PROJECTS } from "@/cache_keys";
import Project from "@/domain/project/entity";

export interface ProjectsResponse extends FeatureCollection<Point | null, ProjectProperties> {
  features: ProjectOutputDTO[];
}

export function useProjects() {
  const apiRequest = useApiRequest();
  const url = __URLS__.ogc_api.base + __URLS__.ogc_api.collections + "/projects/items";

  return useQuery({
    queryKey: CACHE_KEY_PROJECTS,
    queryFn: async () => {
      const data = await apiRequest<ProjectsResponse>(url);
      return data.features.map(dto => new Project(dto));
    },
  });
}
