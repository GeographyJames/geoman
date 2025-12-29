import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { useNavigate, useSearch } from "@tanstack/react-router";
import { Map, View } from "ol";
import TileLayer from "ol/layer/Tile";
import { OSM } from "ol/source";
import { useEffect, useState } from "react";
import { defaults as defaultControls } from "ol/control";
import { CloseButton, ExpandButton } from "@/components/Buttons";
import type Project from "@/domain/project/entity";

export const ProjectPanel = ({ project }: { project: Project }) => {
  const { containerRef, mapRef } = useMapContext();
  const navigate = useNavigate();
  const search = useSearch({ from: "/_app/" });
  const [expanded, setExpanded] = useState<boolean>(() => {
    // Default to expanded on medium screens and up (sm breakpoint is 640px)
    return window.matchMedia("(min-width: 640px)").matches;
  });

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
    <div className="flex  flex-shrink-0 flex-col pointer-events-auto bg-base-100 p-2 min-w-0 w-full max-w-[600px] shadow-lg rounded-box relative  min-w-0">
      <div className="absolute right-2 top-2">
        <CloseButton onClick={handleClose} />
      </div>
      <div className="flex gap-2 items-center">
        <ExpandButton
          expanded={expanded}
          onClick={() => setExpanded(!expanded)}
        />

        <h1 className="text-l font-bold">{project.name}</h1>
      </div>

      {expanded && (
        <div className="flex h-96">
          <p>Project details will go here</p>
        </div>
      )}
    </div>
  );
};
