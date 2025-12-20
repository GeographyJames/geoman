import { useEffect } from "react";
import { useProjects } from "@/hooks/api/useProjects";
import { useMapContext } from "@/contexts/MapRefContext";
import { Map, View } from "ol";
import TileLayer from "ol/layer/Tile";
import { OSM } from "ol/source";
import { defaults as defaultControls } from "ol/control";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import { Feature } from "ol";
import { Point } from "ol/geom";
import { fromLonLat } from "ol/proj";
import { Style, Icon } from "ol/style";

export default function ProjectsMap() {
  const { containerRef, mapRef } = useMapContext();
  const { data: projects } = useProjects();

  useEffect(() => {
    const container = containerRef.current;
    if (!container || !projects) return;

    // Create features from project data
    const features = projects
      .filter((project) => project.outputDto.geometry)
      .map((project) => {
        const [lng, lat] = project.outputDto.geometry!.coordinates;
        const feature = new Feature({
          geometry: new Point(fromLonLat([lng, lat])),
          name: project.name,
        });
        return feature;
      });

    // Create vector source and layer
    const vectorSource = new VectorSource({ features });
    const vectorLayer = new VectorLayer({
      source: vectorSource,
      style: new Style({
        image: new Icon({
          anchor: [0.5, 1],
          src: "https://openlayers.org/en/latest/examples/data/icon.png",
        }),
      }),
    });

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

    // Add projects layer
    map.addLayer(vectorLayer);

    // Fit view to projects
    if (features.length > 0) {
      const extent = vectorSource.getExtent();
      map.getView().fit(extent, {
        padding: [50, 50, 50, 50],
        maxZoom: 16,
        duration: 0,
      });
    }

    console.log("Added projects layer with", features.length, "markers");

    // Cleanup: remove layer when route unmounts (but keep map)
    return () => {
      console.log("Removing projects layer");
      map?.removeLayer(vectorLayer);
    };
  }, [containerRef, mapRef, projects]);

  return <></>;
}
