import { Outlet, useSearch } from "@tanstack/react-router";
import { Search } from "./search/Search";
import { ProjectPanel } from "./project/ProjectPanel";
import { useProjects } from "@/hooks/api/projects/useProjects";
import { useSearchbar } from "../contexts/SearchbarContext";

export const OverlayPanels = () => {
  const { projects } = useSearch({ from: "/_app/" });
  const { data } = useProjects();
  const { isOpen } = useSearchbar();
  const loadedProjects = projects ? projects.split(",") : [];
  if (data) {
    return (
      <div
        id="overlay-panels"
        className="flex flex-col absolute top-0 p-4  pointer-events-none w-full min-w-0 min-h-0 gap-2 h-full"
      >
        <Search />

        {projects &&
          data
            .filter((p) => loadedProjects.includes(p.slug))
            .map((p) => <ProjectPanel key={p.slug} project={p} />)}

        <Outlet />
      </div>
    );
  }
  return null;
};
