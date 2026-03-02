import { useCallback, useEffect, useRef, useState } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import Overlay from "ol/Overlay";
import GeoJSON from "ol/format/GeoJSON";
import { Feature } from "ol";
import { Point } from "ol/geom";
import Style from "ol/style/Style";
import Fill from "ol/style/Fill";
import Stroke from "ol/style/Stroke";
import Text from "ol/style/Text";
import type { StyleLike } from "ol/style/Style";
import type { MapBrowserEvent } from "ol";
import type { TurbineFeature, TurbineFeatureCollection } from "@/hooks/api/projectFeature.ts/useTurbineLayoutGeojson";

export type TurbinePopupData = TurbineFeature & {
  storage_crs_srid: number;
  storage_crs_name: string | null;
};

export function useTurbineLayerWithPopup(
  collection: TurbineFeatureCollection | undefined,
  style: StyleLike,
  showTurbineNumbers: boolean,
) {
  const { mapRef } = useMapContext();
  const layerRef = useRef<VectorLayer | null>(null);
  const overlayRef = useRef<Overlay | null>(null);
  const popupRef = useRef<HTMLDivElement>(null);
  const [popupContent, setPopupContent] = useState<TurbinePopupData | null>(null);
  const showNumbersRef = useRef(showTurbineNumbers);

  useEffect(() => {
    showNumbersRef.current = showTurbineNumbers;
    layerRef.current?.changed();
  }, [showTurbineNumbers]);

  useEffect(() => {
    const map = mapRef.current;
    if (!map || !collection) return;

    const format = new GeoJSON();
    const olFeatures = format.readFeatures(collection, {
      featureProjection: "EPSG:3857",
      dataProjection: "EPSG:4326",
    });
    olFeatures.forEach((olFeature, i) => {
      const data: TurbinePopupData = {
        ...collection.features[i],
        storage_crs_srid: collection.storage_crs_srid,
        storage_crs_name: collection.storage_crs_name,
      };
      olFeature.set("_source", data);
    });

    const source = new VectorSource({ features: olFeatures });
    const base = style as Style;
    const layer = new VectorLayer({
      source,
      zIndex: 10,
      style: (feature) => {
        if (!showNumbersRef.current) return base;
        return new Style({
          stroke: base.getStroke() ?? undefined,
          fill: base.getFill() ?? undefined,
          image: base.getImage() ?? undefined,
          text: new Text({
            text: `T${String(feature.get("turbine_number"))}`,
            offsetX: 10,
            offsetY: -10,
            textAlign: "left",
            textBaseline: "bottom",
            font: "bold 11px sans-serif",
            fill: new Fill({ color: "#1f2937" }),
            stroke: new Stroke({ color: "#fff", width: 3 }),
          }),
        });
      },
    });
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
        setPopupContent(hit.get("_source") as TurbinePopupData);
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
