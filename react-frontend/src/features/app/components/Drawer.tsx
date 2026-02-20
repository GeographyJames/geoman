import BaseMap from "@/features/app/components/map/BaseMap";
import ProjectsMap from "@/components/mapComponents/ProjectsMap";
import { useProjects } from "@/hooks/api/projects/useProjects";
import type Project from "@/domain/project/entity";
import { ProjectsFilterProvider } from "@/features/app/contexts/ProjectsFilterContext";
import { fromLonLat } from "ol/proj";
import { boundingExtent } from "ol/extent";
import { Sidebar } from "./Sidebar";
import { OverlayPanels } from "./OverlayPanels";
import { useSidebar } from "@/features/app/contexts/SidebarContext";
import { useSearchbar } from "../contexts/SearchbarContext";

export const Drawer = () => {
  const sidebar = useSidebar();
  const { data: projects, isLoading } = useProjects();

  if (isLoading || !projects) {
    return (
      <div className="drawer h-full items-center justify-center">
        <span className="loading loading-spinner loading-lg" />
      </div>
    );
  }

  return (
    <ProjectsFilterProvider allProjects={projects}>
      <div className="drawer h-full">
        <input
          id="my-drawer-1"
          type="checkbox"
          className="drawer-toggle"
          checked={sidebar.isOpen}
          onChange={sidebar.toggleSidebar}
        />
        <DrawerMain projects={projects} />
        <DrawerSide />
      </div>
    </ProjectsFilterProvider>
  );
};

const DrawerMain = ({ projects }: { projects: Project[] }) => {
  const { setIsOpen: setSearchOpen } = useSearchbar();

  const coordinates = projects
    .filter((p) => p.centroid)
    .map((p) => fromLonLat(p.centroid!.coordinates));

  const initialExtent = coordinates.length > 0
    ? boundingExtent(coordinates)
    : undefined;

  return (
    <div className="drawer-content h-full">
      <BaseMap
        initialExtent={initialExtent}
        onMouseDown={() => setSearchOpen(false)}
      />
      <ProjectsMap />
      <OverlayPanels />
    </div>
  );
};

const DrawerSide = () => {
  return (
    <div className="drawer-side">
      <label
        htmlFor="my-drawer-1"
        aria-label="close sidebar"
        className="drawer-overlay"
      ></label>
      <Sidebar />
    </div>
  );
};
