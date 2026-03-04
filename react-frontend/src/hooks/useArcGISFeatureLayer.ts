import { useEffect, useRef } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import GeoJSON from "ol/format/GeoJSON";
import { bbox as bboxStrategy } from "ol/loadingstrategy";
import type { MapBrowserEvent } from "ol";
import type { StyleLike } from "ol/style/Style";

/** Adds an ArcGIS FeatureServer layer to the shared map using bbox loading. Adds on mount, removes on unmount. Visibility is toggled via setVisible. */
export function useArcGISFeatureLayer(
  serviceUrl: string,
  style: StyleLike,
  {
    minZoom,
    maxZoom,
    onLoadingChange,
    onFeatureCountChange,
    onFeatureHover,
    visible = true,
  }: {
    minZoom?: number;
    maxZoom?: number;
    onLoadingChange?: (loading: boolean) => void;
    onFeatureCountChange?: (count: number) => void;
    onFeatureHover?: (name: string | null, pixel: [number, number] | null) => void;
    visible?: boolean;
  } = {},
) {
  const { mapRef } = useMapContext();
  const onLoadingChangeRef = useRef(onLoadingChange);
  onLoadingChangeRef.current = onLoadingChange;
  const onFeatureCountChangeRef = useRef(onFeatureCountChange);
  onFeatureCountChangeRef.current = onFeatureCountChange;
  const onFeatureHoverRef = useRef(onFeatureHover);
  onFeatureHoverRef.current = onFeatureHover;
  const layerRef = useRef<VectorLayer | null>(null);

  useEffect(() => {
    const map = mapRef.current;
    if (!map) return;

    const source = new VectorSource({
      format: new GeoJSON(),
      url: (extent) =>
        `${serviceUrl}/query?where=1%3D1` +
        `&geometry=${extent.join(",")}` +
        `&geometryType=esriGeometryEnvelope` +
        `&inSR=3857` +
        `&spatialRel=esriSpatialRelIntersects` +
        `&outSR=4326` +
        `&f=geojson` +
        `&returnGeometry=true`,
      strategy: bboxStrategy,
    });

    source.on("featuresloadstart", () => onLoadingChangeRef.current?.(true));
    source.on("featuresloadend", () => {
      onLoadingChangeRef.current?.(false);
      onFeatureCountChangeRef.current?.(source.getFeatures().length);
    });
    source.on("featuresloaderror", () => onLoadingChangeRef.current?.(false));

    const layer = new VectorLayer({ source, style, minZoom, maxZoom, visible });
    map.getLayers().insertAt(1, layer);
    layerRef.current = layer;

    const cleanups: (() => void)[] = [];

    // When zooming below minZoom, clear the source so stale cached features
    // don't appear alongside freshly loaded ones on the next zoom-in.
    if (minZoom !== undefined) {
      let aboveMinZoom = (map.getView().getZoom() ?? 0) >= minZoom;
      const onMoveEnd = () => {
        const nowAbove = (map.getView().getZoom() ?? 0) >= minZoom!;
        if (aboveMinZoom && !nowAbove) {
          source.clear();
          onFeatureCountChangeRef.current?.(0);
        }
        aboveMinZoom = nowAbove;
      };
      map.on("moveend", onMoveEnd);
      cleanups.push(() => map.un("moveend", onMoveEnd));
    }

    const onPointerMove = (evt: MapBrowserEvent<PointerEvent>) => {
      if (evt.dragging) return;
      let name: string | null = null;
      map.forEachFeatureAtPixel(
        evt.pixel,
        (feature) => {
          name = feature.get("NAME") ?? null;
          return true; // stop after first match
        },
        { layerFilter: (l) => l === layer },
      );
      onFeatureHoverRef.current?.(name, name ? (evt.pixel as [number, number]) : null);
    };
    map.on("pointermove", onPointerMove);
    cleanups.push(() => {
      map.un("pointermove", onPointerMove);
      onFeatureHoverRef.current?.(null, null);
    });

    return () => {
      for (const fn of cleanups) fn();
      map.removeLayer(layer);
      layerRef.current = null;
    };
  }, [mapRef, serviceUrl]);

  useEffect(() => {
    layerRef.current?.setVisible(visible);
  }, [visible]);
}
