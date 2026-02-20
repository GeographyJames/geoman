import { useCallback, useEffect, useRef } from "react";
import { Feature } from "ol";
import type { Style } from "ol/style";

const ANIM_DURATION = 150;

interface UseMarkerAnimationParams {
  activeStyles: Style[][];
  archivedStyles: Style[][];
  hoverScale: number;
  hoveredProjectId: number | null;
  featuresRef: React.RefObject<Map<number, Feature>>;
}

export function useMarkerAnimation({
  activeStyles,
  archivedStyles,
  hoverScale,
  hoveredProjectId,
  featuresRef,
}: UseMarkerAnimationParams) {
  const animFrameIds = useRef<Map<Feature, number>>(new Map());
  const hoveredFeatureRef = useRef<Feature | null>(null);
  const animateScaleRef = useRef<
    ((feature: Feature, from: number, to: number) => void) | null
  >(null);

  // Update the animateScale function when styles change
  useEffect(() => {
    const animSteps = activeStyles.length - 1;

    function animateScale(feature: Feature, from: number, to: number) {
      const styles = feature.get("archived") ? archivedStyles : activeStyles;
      const ids = animFrameIds.current;
      const existing = ids.get(feature);
      if (existing) cancelAnimationFrame(existing);

      const start = performance.now();

      const step = () => {
        const elapsed = performance.now() - start;
        const t = Math.min(elapsed / ANIM_DURATION, 1);
        const eased = t * (2 - t); // ease-out quad
        const scale = from + (to - from) * eased;
        const idx = Math.round(
          ((scale - 1) / (hoverScale - 1)) * animSteps,
        );
        feature.setStyle(styles[Math.max(0, Math.min(animSteps, idx))]);

        if (t < 1) {
          ids.set(feature, requestAnimationFrame(step));
        } else {
          ids.delete(feature);
        }
      };

      ids.set(feature, requestAnimationFrame(step));
    }

    animateScaleRef.current = animateScale;

    return () => {
      animFrameIds.current.forEach((id) => cancelAnimationFrame(id));
      animFrameIds.current.clear();
      animateScaleRef.current = null;
      hoveredFeatureRef.current = null;
    };
  }, [activeStyles, archivedStyles, hoverScale]);

  // Sync list hover → marker animation
  useEffect(() => {
    const animateScale = animateScaleRef.current;
    if (!animateScale) return;

    const prev = hoveredFeatureRef.current;
    if (prev) {
      animateScale(prev, hoverScale, 1);
      hoveredFeatureRef.current = null;
    }

    if (hoveredProjectId !== null) {
      const feature = featuresRef.current?.get(hoveredProjectId);
      if (feature) {
        animateScale(feature, 1, hoverScale);
        hoveredFeatureRef.current = feature;
      }
    }
  }, [hoveredProjectId, hoverScale, featuresRef]);

  const animateScale = useCallback(
    (feature: Feature, from: number, to: number) => {
      animateScaleRef.current?.(feature, from, to);
    },
    [],
  );

  return { animateScale, hoveredFeatureRef, hoverScale };
}
