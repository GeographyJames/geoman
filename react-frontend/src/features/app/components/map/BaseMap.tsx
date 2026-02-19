import { useEffect, useRef } from "react";
import { Map, View } from "ol";
import type { Extent } from "ol/extent";
import TileLayer from "ol/layer/Tile";
import { OSM } from "ol/source";
import { defaults as defaultControls } from "ol/control";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import "ol/ol.css";

interface BaseMapProps {
  initialExtent?: Extent;
  onMouseDown?: () => void;
}

export default function BaseMap({ initialExtent, onMouseDown }: BaseMapProps) {
  const { containerRef, mapRef } = useMapContext();
  const divRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (divRef.current) {
      containerRef.current = divRef.current;
    }

    return () => {
      containerRef.current = null;
    };
  }, [containerRef]);

  useEffect(() => {
    const container = containerRef.current;
    // Don't create if div isn't ready or map already exists
    if (!container || mapRef.current) return;

    const map = new Map({
      target: container,
      controls: defaultControls({
        rotate: false,
        zoom: false,
        attribution: true,
      }),
      layers: [new TileLayer({ source: new OSM() })],
      view: new View({
        center: [0, 0],
        zoom: 2,
      }),
    });

    if (initialExtent) {
      map.getView().fit(initialExtent, {
        padding: [50, 50, 50, 50],
        maxZoom: 16,
      });
    }

    mapRef.current = map;
  }, [containerRef, mapRef, initialExtent]);

  return (
    <div
      ref={divRef}
      className="w-full h-full"
      style={{ backgroundColor: "#f0f0f0" }}
      onMouseDown={onMouseDown}
    />
  );
}
