import { Outlet, useSearch } from "@tanstack/react-router";
import { Search } from "./search/Search";
import { ProjectPanel } from "./project/ProjectPanel";

export const OverlayPanels = () => {
  const { projects } = useSearch({ from: "/_app/" });
  return (
    <div
      id="overlay-panels"
      className="flex flex-col gap-2 absolute top-0 p-4 h-full  pointer-events-none w-full min-w-0"
    >
      <Search />

      {projects && (
        <div className="flex-1 flex flex-col gap-2 overflow-y-auto">
          {projects
            .split(",")
            .filter(Boolean)
            .map((p) => (
              <ProjectPanel key={p} project={p} />
            ))}
        </div>
      )}

      <Outlet />
    </div>
  );
};
