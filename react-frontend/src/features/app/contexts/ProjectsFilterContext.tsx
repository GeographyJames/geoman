import { createContext, useContext, useMemo, useState, type ReactNode } from "react";
import type Project from "@/domain/project/entity";

interface ProjectsFilterContextType {
  projects: Project[];
  showArchivedProjects: boolean;
  setShowArchivedProjects: (value: boolean) => void;
  hoveredProjectId: number | null;
  setHoveredProjectId: (id: number | null) => void;
}

const ProjectsFilterContext = createContext<ProjectsFilterContextType | undefined>(undefined);

export function ProjectsFilterProvider({
  allProjects,
  children,
}: {
  allProjects: Project[];
  children: ReactNode;
}) {
  const [showArchivedProjects, setShowArchivedProjects] = useState(false);
  const [hoveredProjectId, setHoveredProjectId] = useState<number | null>(null);

  const projects = useMemo(
    () => allProjects.filter((p) => showArchivedProjects || !p.archived),
    [allProjects, showArchivedProjects],
  );

  return (
    <ProjectsFilterContext.Provider
      value={{ projects, showArchivedProjects, setShowArchivedProjects, hoveredProjectId, setHoveredProjectId }}
    >
      {children}
    </ProjectsFilterContext.Provider>
  );
}

export function useProjectsFilter() {
  const context = useContext(ProjectsFilterContext);
  if (context === undefined) {
    throw new Error(
      "useProjectsFilter must be used within a ProjectsFilterProvider",
    );
  }
  return context;
}
