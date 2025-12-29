import { Outlet, useSearch } from "@tanstack/react-router";
import { Search } from "./search/Search";
import { ProjectPanel } from "./project/ProjectPanel";
import { useProjects } from "@/hooks/api/projects/useProjects";

export const OverlayPanels = () => {
  const { projects } = useSearch({ from: "/_app/" });
  const { data } = useProjects();
  const loadedProjects = projects ? projects.split(",") : [];
  if (data) {
    return (
      <div
        id="overlay-panels"
        className="flex flex-col gap-2 absolute top-0 p-4 h-full  pointer-events-none w-full min-w-0"
      >
        <div className="flex flex-col min-h-0">
          <Search />
        </div>

        {projects && (
          <div className="flex flex-col gap-2 overflow-y-auto">
            {data
              .filter((p) => loadedProjects.includes(p.slug))
              .map((p) => (
                <ProjectPanel key={p.slug} project={p} />
              ))}
          </div>
        )}

        <Outlet />
      </div>
    );
  }
  return null;
};
