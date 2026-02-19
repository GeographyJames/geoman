import BaseMap from "@/features/app/components/map/BaseMap";
import ProjectsMap from "@/features/projects/components/ProjectsMap";
import { useProjects } from "@/hooks/api/projects/useProjects";
import { ProjectsFilterProvider } from "@/features/app/contexts/ProjectsFilterContext";
import { fromLonLat } from "ol/proj";
import { boundingExtent } from "ol/extent";
import { Sidebar } from "./Sidebar";
import { OverlayPanels } from "./OverlayPanels";
import { useSidebar } from "@/features/app/contexts/SidebarContext";
import { useSearchbar } from "../contexts/SearchbarContext";

export const Drawer = () => {
  const sidebar = useSidebar();
  return (
    <div className="drawer h-full">
      <input
        id="my-drawer-1"
        type="checkbox"
        className="drawer-toggle"
        checked={sidebar.isOpen}
        onChange={sidebar.toggleSidebar}
      />
      <DrawerMain />
      <DrawerSide />
    </div>
  );
};

const DrawerMain = () => {
  const { setIsOpen: setSearchOpen } = useSearchbar();
  const { data: projects, isLoading } = useProjects();

  if (isLoading || !projects) {
    return (
      <div className="drawer-content h-full flex items-center justify-center">
        <span className="loading loading-spinner loading-lg" />
      </div>
    );
  }

  const coordinates = projects
    .filter((p) => p.centroid)
    .map((p) => fromLonLat(p.centroid!.coordinates));

  const initialExtent = coordinates.length > 0
    ? boundingExtent(coordinates)
    : undefined;

  return (
    <ProjectsFilterProvider allProjects={projects}>
      <div className="drawer-content h-full">
        <BaseMap
          initialExtent={initialExtent}
          onMouseDown={() => setSearchOpen(false)}
        />
        <ProjectsMap />
        <OverlayPanels />
      </div>
    </ProjectsFilterProvider>
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
