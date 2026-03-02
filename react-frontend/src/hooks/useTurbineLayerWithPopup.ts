import { useCallback, useEffect, useRef, useState } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import Overlay from "ol/Overlay";
import GeoJSON from "ol/format/GeoJSON";
import { Feature } from "ol";
import { Point } from "ol/geom";
import type { StyleLike } from "ol/style/Style";
import type { MapBrowserEvent } from "ol";

export interface TurbinePopupContent {
  turbineNumber: number;
  hubHeightMm: number | null;
  rotorDiameterMm: number | null;
}

export function useTurbineLayerWithPopup(
  collection: GeoJSON.FeatureCollection | undefined,
  style: StyleLike,
) {
  const { mapRef } = useMapContext();
  const layerRef = useRef<VectorLayer | null>(null);
  const overlayRef = useRef<Overlay | null>(null);
  const popupRef = useRef<HTMLDivElement>(null);
  const [popupContent, setPopupContent] = useState<TurbinePopupContent | null>(null);

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !collection) return;

    const format = new GeoJSON();
    const olFeatures = format.readFeatures(collection, {
      featureProjection: "EPSG:3857",
      dataProjection: "EPSG:4326",
    });

    const source = new VectorSource({ features: olFeatures });
    const layer = new VectorLayer({ source, style });
    map.getLayers().insertAt(1, layer);
    layerRef.current = layer;

    return () => {
      map.removeLayer(layer);
      layerRef.current = null;
      overlayRef.current?.setPosition(undefined);
      setPopupContent(null);
    };
  }, [mapRef, collection]);

  useEffect(() => {
    const map = mapRef.current;
    const popupEl = popupRef.current;
    if (!map || !popupEl) return;

    const overlay = new Overlay({
      element: popupEl,
      positioning: "bottom-center",
      offset: [0, -10],
      autoPan: true,
    });
    map.addOverlay(overlay);
    overlayRef.current = overlay;

    const handleClick = (e: MapBrowserEvent) => {
      const layer = layerRef.current;
      if (!layer) return;

      const hit = map.forEachFeatureAtPixel(e.pixel, (f) => f, {
        layerFilter: (l) => l === layer,
      });

      if (hit instanceof Feature) {
        const geom = hit.getGeometry() as Point;
        setPopupContent({
          turbineNumber: hit.get("turbine_number"),
          hubHeightMm: hit.get("hub_height_mm") ?? null,
          rotorDiameterMm: hit.get("rotor_diameter_mm") ?? null,
        });
        overlay.setPosition(geom.getCoordinates());
      } else {
        overlay.setPosition(undefined);
        setPopupContent(null);
      }
    };

    map.on("click", handleClick);

    return () => {
      map.removeOverlay(overlay);
      map.un("click", handleClick);
      overlayRef.current = null;
    };
  }, [mapRef]);

  const closePopup = useCallback(() => {
    overlayRef.current?.setPosition(undefined);
    setPopupContent(null);
  }, []);

  return { popupRef, popupContent, closePopup };
}
