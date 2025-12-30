import { useMapContext } from "@/features/app/contexts/MapRefContext";
import { Map, View } from "ol";
import TileLayer from "ol/layer/Tile";
import { OSM } from "ol/source";
import { useEffect } from "react";
import { defaults as defaultControls } from "ol/control";

export const ProjectAdmin = () => {
  const { containerRef, mapRef } = useMapContext();
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
  return <>Project Admin</>;
};
