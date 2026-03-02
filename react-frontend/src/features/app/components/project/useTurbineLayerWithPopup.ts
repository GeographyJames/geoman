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
  layout_id: number;
  layout_name: string;
};

export function useTurbineLayerWithPopup(
  collection: TurbineFeatureCollection | undefined,
  style: StyleLike,
  showTurbineNumbers: boolean,
  layoutId: number,
  layoutName: string,
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
    const popupEl = popupRef.current;
    if (!map || !collection || !popupEl) return;

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
        layout_id: layoutId,
        layout_name: layoutName,
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
    layer.set("interactive", true);
    map.getLayers().insertAt(1, layer);
    layerRef.current = layer;

    const overlay = new Overlay({
      element: popupEl,
      positioning: "bottom-center",
      offset: [0, -10],
      autoPan: true,
    });
    map.addOverlay(overlay);
    overlayRef.current = overlay;

    const handleClick = (e: MapBrowserEvent) => {
      const topHit = map.forEachFeatureAtPixel(e.pixel, (f, l) => ({ feature: f, layer: l }));
      if (topHit?.layer === layer && topHit.feature instanceof Feature) {
        const geom = topHit.feature.getGeometry() as Point;
        setPopupContent(topHit.feature.get("_source") as TurbinePopupData);
        overlay.setPosition(geom.getCoordinates());
      } else {
        overlay.setPosition(undefined);
        setPopupContent(null);
      }
    };

    const handlePointerMove = (e: MapBrowserEvent) => {
      const topLayer = map.forEachFeatureAtPixel(e.pixel, (f, l) => l);
      if (topLayer === layer) {
        map.getTargetElement().style.cursor = "pointer";
      } else if (!topLayer || !topLayer.get("interactive")) {
        map.getTargetElement().style.cursor = "";
      }
    };

    map.on("click", handleClick);
    map.on("pointermove", handlePointerMove);

    return () => {
      map.removeLayer(layer);
      map.removeOverlay(overlay);
      map.un("click", handleClick);
      map.un("pointermove", handlePointerMove);
      map.getTargetElement().style.cursor = "";
      layerRef.current = null;
      overlayRef.current = null;
      setPopupContent(null);
    };
  }, [mapRef, collection]);

  const closePopup = useCallback(() => {
    overlayRef.current?.setPosition(undefined);
    setPopupContent(null);
  }, []);

  return { popupRef, popupContent, closePopup };
}
