import { useEffect, useRef } from "react";
import type { FeatureCollection } from "geojson";
import Map from "ol/Map";
import View from "ol/View";
import TileLayer from "ol/layer/Tile";
import OSM from "ol/source/OSM";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import GeoJSON from "ol/format/GeoJSON";
import { isEmpty } from "ol/extent";
import { Style, Stroke, Fill, Circle } from "ol/style";
import "ol/ol.css";

interface ShapefilePreviewProps {
  geojson: FeatureCollection | null;
  prj: string | null;
  nullGeometryCount: number;
}

function parseCrsName(prj: string): string {
  const match = prj.match(/^[A-Z]+\["([^"]+)"/);
  return match ? match[1] : prj.slice(0, 40);
}

export function ShapefilePreview({ geojson, prj, nullGeometryCount }: ShapefilePreviewProps) {
  const mapRef = useRef<HTMLDivElement>(null);
  const mapInstanceRef = useRef<Map | null>(null);

  useEffect(() => {
    if (!geojson || !mapRef.current) {
      if (mapInstanceRef.current) {
        mapInstanceRef.current.setTarget(undefined);
        mapInstanceRef.current = null;
      }
      return;
    }

    const format = new GeoJSON();
    const features = format.readFeatures(geojson, {
      featureProjection: "EPSG:3857",
    });

    const vectorSource = new VectorSource({ features });

    const vectorLayer = new VectorLayer({
      source: vectorSource,
      style: new Style({
        stroke: new Stroke({ color: "#3b82f6", width: 2 }),
        fill: new Fill({ color: "rgba(59, 130, 246, 0.15)" }),
        image: new Circle({
          radius: 6,
          fill: new Fill({ color: "#3b82f6" }),
          stroke: new Stroke({ color: "#fff", width: 2 }),
        }),
      }),
    });

    const map = new Map({
      target: mapRef.current,
      layers: [new TileLayer({ source: new OSM() }), vectorLayer],
      view: new View({ center: [0, 0], zoom: 2 }),
    });

    const extent = vectorSource.getExtent();
    if (!isEmpty(extent)) {
      map.getView().fit(extent, { padding: [40, 40, 40, 40], maxZoom: 18 });
    }

    mapInstanceRef.current = map;

    return () => {
      map.setTarget(undefined);
      mapInstanceRef.current = null;
    };
  }, [geojson]);

  if (!geojson) return null;

  const geometryType = geojson.features[0]?.geometry.type ?? "Unknown";
  const crs = prj ? parseCrsName(prj) : "Unknown";

  return (
    <div className="space-y-2">
      <div
        ref={mapRef}
        className="w-full rounded-lg border border-base-300 overflow-hidden"
        style={{ height: 300 }}
      />
      <div className="flex flex-wrap gap-4 text-sm bg-base-200 rounded-lg px-3 py-2">
        <span><span className="font-semibold">CRS:</span> {crs}</span>
        <span><span className="font-semibold">Geometry:</span> {geometryType}</span>
        <span><span className="font-semibold">Features:</span> {geojson.features.length}</span>
        {nullGeometryCount > 0 && (
          <span className="text-warning font-semibold">
            {nullGeometryCount} feature{nullGeometryCount > 1 ? "s" : ""} without geometry will be filtered out
          </span>
        )}
      </div>
    </div>
  );
}
