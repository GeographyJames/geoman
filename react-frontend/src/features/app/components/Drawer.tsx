import BaseMap from "@/features/app/components/map/BaseMap";
import ProjectsMap from "@/components/mapComponents/ProjectsMap";
import DevelopableAreasLayer from "@/components/mapComponents/DevelopableAreasLayer";
import { useProjects } from "@/hooks/api/projects/useProjects";
import type Project from "@/domain/project/entity";
import { ProjectsFilterProvider } from "@/features/app/contexts/ProjectsFilterContext";
import { fromLonLat } from "ol/proj";
import { boundingExtent } from "ol/extent";
import { Sidebar } from "./Sidebar";
import { OverlayPanels } from "./OverlayPanels";
import { useSearchbarSetOpen } from "../contexts/SearchbarContext";
import { memo, useMemo, useState } from "react";

export const Drawer = () => {
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
          readOnly
        />
        <DrawerMain projects={projects} />
        <DrawerSide />
      </div>
    </ProjectsFilterProvider>
  );
};

const DrawerMain = memo(({ projects }: { projects: Project[] }) => {
  const setSearchOpen = useSearchbarSetOpen();
  const [showDevelopableAreas, setShowDevelopableAreas] = useState(true);
  const coordinates = useMemo(
    () =>
      projects
        .filter((p) => p.centroid)
        .map((p) => fromLonLat(p.centroid!.coordinates)),
    [projects],
  );
  const initialExtent = useMemo(
    () => (coordinates.length > 0 ? boundingExtent(coordinates) : undefined),
    [coordinates],
  );

  return (
    <div className="drawer-content h-full">
      <BaseMap
        initialExtent={initialExtent}
        onMouseDown={() => setSearchOpen(false)}
      />
      {showDevelopableAreas && <DevelopableAreasLayer />}
      <ProjectsMap />
      <OverlayPanels />
      <div className="absolute top-4 right-4 bg-base-100 rounded-box shadow-md p-3 flex flex-col gap-2 pointer-events-auto">
        <label className="flex items-center gap-2 cursor-pointer text-sm">
          <input
            type="checkbox"
            className="checkbox checkbox-sm checkbox-primary"
            checked={showDevelopableAreas}
            onChange={(e) => setShowDevelopableAreas(e.target.checked)}
          />
          <svg width="20" height="14" className="shrink-0">
            <rect
              x="1" y="1" width="18" height="12" rx="2"
              fill="rgba(37, 99, 235, 0.2)"
              stroke="#2563EB"
              strokeWidth="1.5"
            />
          </svg>
          Developable Areas
        </label>
      </div>
    </div>
  );
});

const DrawerSide = memo(() => {
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
});
