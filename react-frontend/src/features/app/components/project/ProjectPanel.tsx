import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { useNavigate } from "@tanstack/react-router";
import { Map, View } from "ol";
import TileLayer from "ol/layer/Tile";
import { OSM } from "ol/source";
import { useEffect } from "react";
import { defaults as defaultControls } from "ol/control";

export const ProjectPanel = () => {
  const { containerRef, mapRef } = useMapContext();
  const navigate = useNavigate();

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
      className="flex  flex-shrink-0 flex-col pointer-events-auto bg-white p-4 min-w-0 w-full max-w-[600px] shadow-lg rounded-box relative  min-w-0"
    >
      <button
        onClick={() => navigate({ to: "/" })}
        className="absolute top-2 right-2 btn btn-sm btn-ghost btn-circle"
      >
        ✕
      </button>
      <h1 className="text-xl font-bold">Project: todo!</h1>
      <p>Project details will go here</p>
    </div>
  );
};
