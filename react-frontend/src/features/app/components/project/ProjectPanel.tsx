import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { useNavigate, useSearch } from "@tanstack/react-router";
import { Map, View } from "ol";
import TileLayer from "ol/layer/Tile";
import { OSM } from "ol/source";
import { useEffect } from "react";
import { defaults as defaultControls } from "ol/control";
import { CloseButton } from "@/components/Buttons";
import type Project from "@/domain/project/entity";

export const ProjectPanel = ({ project }: { project: Project }) => {
  const { containerRef, mapRef } = useMapContext();
  const navigate = useNavigate();
  const search = useSearch({ from: "/_app/" });

  // Default to expanded on medium screens and up (sm breakpoint is 640px)
  const defaultExpanded = window.matchMedia("(min-width: 640px)").matches;

  const handleClose = () => {
    const currentProjects = search.projects || "";
    const projectsArray = currentProjects.split(",").filter(Boolean);
    const updatedProjects = projectsArray
      .filter((p) => p !== project.slug)
      .join(",");

    navigate({
      from: "/",
      search: { ...search, projects: updatedProjects || undefined },
    });
  };

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;
    // Get or create the map
    let map = mapRef.current;

    if (!map) {
      // Map doesn't exist - create it
      const osmLayer = new TileLayer({ source: new OSM() });
      map = new Map({
        target: container,
        controls: defaultControls({
          rotate: false,
          zoom: false,
          attribution: true,
        }),
        layers: [osmLayer],
        view: new View({
          center: [0, 0],
          zoom: 2,
        }),
      });
      mapRef.current = map;
      console.log("Created new map instance");
    }
  }, [containerRef, mapRef]);

  return (
    <details
      className="collapse collapse-arrow flex-shrink-0 pointer-events-auto bg-base-100 min-w-0 w-full max-w-[600px] shadow-lg rounded-box relative"
      open={defaultExpanded}
    >
      <summary className="collapse-title after:start-5 after:end-auto ps-12 text-l font-bold pr-4 flex justify-between items-center py-2">
        {project.name}
        <div className="">
          <CloseButton onClick={handleClose} />
        </div>
      </summary>

      <div className="collapse-content">
        <div className="flex h-96">
          <p>Project details will go here</p>
        </div>
      </div>
    </details>
  );
};
