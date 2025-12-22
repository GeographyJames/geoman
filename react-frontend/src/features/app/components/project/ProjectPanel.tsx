import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { useNavigate } from "@tanstack/react-router";
import { Map, View } from "ol";
import TileLayer from "ol/layer/Tile";
import { OSM } from "ol/source";
import { useEffect, useState } from "react";
import { defaults as defaultControls } from "ol/control";
import { CloseButton, ExpandButton } from "@/components/Buttons";

export const ProjectPanel = () => {
  const { containerRef, mapRef } = useMapContext();
  const navigate = useNavigate();
  const [expanded, setExpanded] = useState<boolean>(() => {
    // Default to expanded on medium screens and up (sm breakpoint is 640px)
    return window.matchMedia("(min-width: 640px)").matches;
  });

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
    <div
      id="outlet"
      className="flex  flex-shrink-0 flex-col pointer-events-auto bg-base-100 p-2 min-w-0 w-full max-w-[600px] shadow-lg rounded-box relative  min-w-0"
    >
      <div className="absolute right-2 top-2">
        <CloseButton onClick={() => navigate({ to: "/" })} />
      </div>
      <div className="flex gap-2 items-center">
        <ExpandButton
          expanded={expanded}
          onClick={() => setExpanded(!expanded)}
        />
        <h1 className="text-l font-bold">Project: todo!</h1>
      </div>

      {expanded && <p>Project details will go here</p>}
    </div>
  );
};
