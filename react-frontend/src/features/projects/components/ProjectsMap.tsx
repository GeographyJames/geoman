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
  const { projects } = useProjectsFilter();
  const popupRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);

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
        feature.set("projectName", p.name);
        feature.set("projectUrl", p.url);
        feature.set("archived", p.archived);
        return feature;
      });

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

    const HOVER_SCALE = 1.15;
    const ANIM_DURATION = 150;
    const ANIM_STEPS = 8;

    // Pre-build styles at discrete scale steps for each colour
    const scaleSteps = Array.from({ length: ANIM_STEPS + 1 }, (_, i) =>
      1 + (HOVER_SCALE - 1) * (i / ANIM_STEPS),
    );
    const activeStyles = scaleSteps.map((s) => makeStyles("#2A81CB", s));
    const archivedStyles = scaleSteps.map((s) => makeStyles("#9CA3AF", s));

    const vectorLayer = new VectorLayer({
      source: new VectorSource({ features }),
      style: (feature) =>
        feature.get("archived") ? archivedStyles[0] : activeStyles[0],
    });

    let animFrameId: number | null = null;

    function animateScale(
      feature: Feature,
      from: number,
      to: number,
    ) {
      const styles = feature.get("archived") ? archivedStyles : activeStyles;
      const start = performance.now();

      const step = () => {
        const elapsed = performance.now() - start;
        const t = Math.min(elapsed / ANIM_DURATION, 1);
        const eased = t * (2 - t); // ease-out quad
        const scale = from + (to - from) * eased;
        const idx = Math.round(((scale - 1) / (HOVER_SCALE - 1)) * ANIM_STEPS);
        feature.setStyle(styles[Math.max(0, Math.min(ANIM_STEPS, idx))]);

        if (t < 1) {
          animFrameId = requestAnimationFrame(step);
        }
      };

      if (animFrameId) cancelAnimationFrame(animFrameId);
      animFrameId = requestAnimationFrame(step);
    }

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

    let hoveredFeature: Feature | null = null;

    const handlePointerMove = (e: MapBrowserEvent<UIEvent>) => {
      const hit = map.forEachFeatureAtPixel(e.pixel, (f) => f, {
        layerFilter: (layer) => layer === vectorLayer,
      });

      if (hoveredFeature && hoveredFeature !== hit) {
        animateScale(hoveredFeature, HOVER_SCALE, 1);
        hoveredFeature = null;
      }

      if (hit && hit instanceof Feature && hit !== hoveredFeature) {
        animateScale(hit, 1, HOVER_SCALE);
        hoveredFeature = hit;
      }

      map.getTargetElement().style.cursor = hit ? "pointer" : "";
    };

    map.on("click", handleClick);
    map.on("pointermove", handlePointerMove);

    return () => {
      if (animFrameId) cancelAnimationFrame(animFrameId);
      map.removeLayer(vectorLayer);
      map.removeOverlay(overlay);
      map.un("click", handleClick);
      map.un("pointermove", handlePointerMove);
      map.getTargetElement().style.cursor = "";
    };
  }, [mapRef, projects]);

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
