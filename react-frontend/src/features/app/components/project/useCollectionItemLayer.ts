import { useMemo } from "react";
import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";
import { useFeatureCollectionLayer, useFeatureLayer, useZoomToFeature } from "@/hooks/useFeatureLayer";
import { useTurbineLayoutGeojson } from "@/hooks/api/projectFeature.ts/useTurbineLayoutGeojson";
import { useTurbineLayerWithPopup } from "@/features/app/components/project/useTurbineLayerWithPopup";
import { type WakePreset, generateTurbineAreas } from "@/lib/turbineAreas";
import { primaryStyle, defaultStyle, sweptAreaStyle, wakeEllipseStyle } from "./featureStyles";
import { TURBINE_LAYOUTS_COLLECTION_ID } from "@/constants";

export function useCollectionItemLayer(
  item: ProjectCollectionItem,
  {
    visible,
    areasVisible,
    wakePreset,
    windFromDeg,
    showTurbineNumbers,
    projectSlug,
    collectionSlug,
  }: {
    visible: boolean;
    areasVisible: boolean;
    wakePreset: WakePreset;
    windFromDeg: number;
    showTurbineNumbers: boolean;
    projectSlug: string;
    collectionSlug: string;
  },
) {
  const isTurbineLayout = item.properties.collection_id === TURBINE_LAYOUTS_COLLECTION_ID;

  const { data: turbineGeojson } = useTurbineLayoutGeojson(
    projectSlug,
    collectionSlug,
    item.id,
    visible && isTurbineLayout,
  );

  const style = item.properties.is_primary ? primaryStyle : defaultStyle;
  useFeatureLayer(visible && !isTurbineLayout ? item : undefined, style);

  const { popupPixel, popupContent, closePopup } = useTurbineLayerWithPopup(
    visible && isTurbineLayout ? turbineGeojson : undefined,
    style,
    showTurbineNumbers,
    item.id,
    item.properties.name,
  );

  const turbineAreas = useMemo(
    () =>
      visible && areasVisible && turbineGeojson
        ? generateTurbineAreas(turbineGeojson, wakePreset, windFromDeg)
        : null,
    [visible, areasVisible, wakePreset, windFromDeg, turbineGeojson],
  );
  useFeatureCollectionLayer(turbineAreas?.sweptAreas, sweptAreaStyle);
  useFeatureCollectionLayer(turbineAreas?.wakeEllipses, wakeEllipseStyle);

  const zoomToFeature = useZoomToFeature(item);

  return { popupPixel, popupContent, closePopup, zoomToFeature };
}
