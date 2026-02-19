import { useEffect, useRef } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import Overlay from "ol/Overlay";
import { Feature } from "ol";
import { Point } from "ol/geom";
import { fromLonLat } from "ol/proj";
import { Style, Icon } from "ol/style";
import { useProjectsFilter } from "@/features/app/contexts/ProjectsFilterContext";
import MapPopup from "@/features/app/components/map/MapPopup";
import type { MapBrowserEvent } from "ol";

export default function ProjectsMap() {
  const { mapRef } = useMapContext();
  const { projects, hoveredProjectId } = useProjectsFilter();
  const popupRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);
  const featuresRef = useRef<Map<number, Feature>>(new Map());
  const animRef = useRef<{
    animFrameIds: Map<Feature, number>;
    hoveredFeature: Feature | null;
    animateScale: ((feature: Feature, from: number, to: number) => void) | null;
    HOVER_SCALE: number;
  }>({
    animFrameIds: new Map(),
    hoveredFeature: null,
    animateScale: null,
    HOVER_SCALE: 1.15,
  });

  useEffect(() => {
    const map = mapRef.current;
    const popupEl = popupRef.current;
    const contentEl = contentRef.current;
    if (!map || !popupEl || !contentEl) return;

    const features = projects
      .filter((p) => p.centroid)
      .map((p) => {
        const [lng, lat] = p.centroid!.coordinates;
        const feature = new Feature({
          geometry: new Point(fromLonLat([lng, lat])),
        });
        feature.set("projectId", p.id);
        feature.set("projectName", p.name);
        feature.set("projectUrl", p.url);
        feature.set("archived", p.archived);
        return feature;
      });

    const featureMap = new Map<number, Feature>();
    features.forEach((f) => featureMap.set(f.get("projectId"), f));
    featuresRef.current = featureMap;

    const markerSvg = (fill: string) =>
      `data:image/svg+xml,${encodeURIComponent(
        `<svg xmlns="http://www.w3.org/2000/svg" width="25" height="41" viewBox="0 0 25 41">` +
          `<path d="M12.5 0C5.6 0 0 5.6 0 12.5c0 2.4.7 4.6 1.9 6.5L12.5 41l10.6-22C24.3 17.1 25 14.9 25 12.5 25 5.6 19.4 0 12.5 0z" fill="${fill}"/>` +
          `<circle cx="12.5" cy="12.5" r="5" fill="#fff" opacity="0.9"/>` +
          `</svg>`,
      )}`;

    const makeStyles = (fill: string, scale = 1) => [
      new Style({
        image: new Icon({
          anchor: [0.3, 1],
          scale,
          src: "/images/marker-shadow.png",
        }),
      }),
      new Style({
        image: new Icon({
          anchor: [0.5, 1],
          scale,
          src: markerSvg(fill),
        }),
      }),
    ];

    const HOVER_SCALE = 2;
    const ANIM_DURATION = 150;
    const ANIM_STEPS = 8;

    // Pre-build styles at discrete scale steps for each colour
    const scaleSteps = Array.from(
      { length: ANIM_STEPS + 1 },
      (_, i) => 1 + (HOVER_SCALE - 1) * (i / ANIM_STEPS),
    );
    const activeStyles = scaleSteps.map((s) => makeStyles("#2A81CB", s));
    const archivedStyles = scaleSteps.map((s) => makeStyles("#9CA3AF", s));

    const vectorLayer = new VectorLayer({
      source: new VectorSource({ features }),
      style: (feature) =>
        feature.get("archived") ? archivedStyles[0] : activeStyles[0],
    });

    function animateScale(feature: Feature, from: number, to: number) {
      const styles = feature.get("archived") ? archivedStyles : activeStyles;
      const { animFrameIds } = animRef.current;
      const existing = animFrameIds.get(feature);
      if (existing) cancelAnimationFrame(existing);

      const start = performance.now();

      const step = () => {
        const elapsed = performance.now() - start;
        const t = Math.min(elapsed / ANIM_DURATION, 1);
        const eased = t * (2 - t); // ease-out quad
        const scale = from + (to - from) * eased;
        const idx = Math.round(((scale - 1) / (HOVER_SCALE - 1)) * ANIM_STEPS);
        feature.setStyle(styles[Math.max(0, Math.min(ANIM_STEPS, idx))]);

        if (t < 1) {
          animFrameIds.set(feature, requestAnimationFrame(step));
        } else {
          animFrameIds.delete(feature);
        }
      };

      animFrameIds.set(feature, requestAnimationFrame(step));
    }

    animRef.current.animateScale = animateScale;
    animRef.current.HOVER_SCALE = HOVER_SCALE;

    const overlay = new Overlay({
      element: popupEl,
      positioning: "bottom-center",
      offset: [0, -45],
      autoPan: true,
    });

    map.addLayer(vectorLayer);
    map.addOverlay(overlay);

    const handleClick = (e: MapBrowserEvent<UIEvent>) => {
      const hit = map.forEachFeatureAtPixel(e.pixel, (feature) => feature, {
        layerFilter: (layer) => layer === vectorLayer,
      });

      if (hit && hit instanceof Feature) {
        const geom = hit.getGeometry() as Point;
        const name = hit.get("projectName") as string;
        const url = hit.get("projectUrl") as string;
        contentEl.innerHTML = `<a href="${url}" class="link font-semibold">${name}</a>`;
        overlay.setPosition(geom.getCoordinates());
      } else {
        overlay.setPosition(undefined);
      }
    };

    const handlePointerMove = (e: MapBrowserEvent<UIEvent>) => {
      const hit = map.forEachFeatureAtPixel(e.pixel, (f) => f, {
        layerFilter: (layer) => layer === vectorLayer,
      });

      const { hoveredFeature } = animRef.current;

      if (hoveredFeature && hoveredFeature !== hit) {
        animateScale(hoveredFeature, HOVER_SCALE, 1);
        animRef.current.hoveredFeature = null;
      }

      if (hit && hit instanceof Feature && hit !== hoveredFeature) {
        animateScale(hit, 1, HOVER_SCALE);
        animRef.current.hoveredFeature = hit;
      }

      map.getTargetElement().style.cursor = hit ? "pointer" : "";
    };

    map.on("click", handleClick);
    map.on("pointermove", handlePointerMove);

    return () => {
      animRef.current.animFrameIds.forEach((id) => cancelAnimationFrame(id));
      animRef.current.animFrameIds.clear();
      animRef.current.animateScale = null;
      animRef.current.hoveredFeature = null;
      featuresRef.current.clear();
      map.removeLayer(vectorLayer);
      map.removeOverlay(overlay);
      map.un("click", handleClick);
      map.un("pointermove", handlePointerMove);
      map.getTargetElement().style.cursor = "";
    };
  }, [mapRef, projects]);

  useEffect(() => {
    const { animateScale, hoveredFeature, HOVER_SCALE } = animRef.current;
    if (!animateScale) return;

    if (hoveredFeature) {
      animateScale(hoveredFeature, HOVER_SCALE, 1);
      animRef.current.hoveredFeature = null;
    }

    if (hoveredProjectId !== null) {
      const feature = featuresRef.current.get(hoveredProjectId);
      if (feature) {
        animateScale(feature, 1, HOVER_SCALE);
        animRef.current.hoveredFeature = feature;
      }
    }
  }, [hoveredProjectId]);

  const closePopup = () => {
    mapRef.current
      ?.getOverlays()
      .getArray()
      .forEach((o) => o.setPosition(undefined));
  };

  return (
    <MapPopup ref={popupRef} contentRef={contentRef} onClose={closePopup} />
  );
}
