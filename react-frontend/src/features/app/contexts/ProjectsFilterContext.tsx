import { createContext, useContext, useMemo, useState, type ReactNode } from "react";
import type Project from "@/domain/project/entity";

interface ProjectsFilterContextType {
  allProjects: Project[];
  projects: Project[];
  showArchivedProjects: boolean;
  setShowArchivedProjects: (value: boolean) => void;
}

const ProjectsFilterContext = createContext<ProjectsFilterContextType | undefined>(undefined);

// State: only ProjectsMap/MarkerAnimation subscribes (OK to re-render on hover)
const HoveredProjectStateContext = createContext<number | null>(null);

// Actions: stable useState setter — consumers NEVER re-render on hover
const HoveredProjectActionsContext = createContext<(id: number | null) => void>(() => {});

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

  const contextValue = useMemo(
    () => ({ allProjects, projects, showArchivedProjects, setShowArchivedProjects }),
    [allProjects, projects, showArchivedProjects, setShowArchivedProjects],
  );

  return (
    <HoveredProjectActionsContext.Provider value={setHoveredProjectId}>
      <HoveredProjectStateContext.Provider value={hoveredProjectId}>
        <ProjectsFilterContext.Provider value={contextValue}>
          {children}
        </ProjectsFilterContext.Provider>
      </HoveredProjectStateContext.Provider>
    </HoveredProjectActionsContext.Provider>
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

export function useHoveredProjectId() {
  return useContext(HoveredProjectStateContext);
}

export function useSetHoveredProject() {
  return useContext(HoveredProjectActionsContext);
}
