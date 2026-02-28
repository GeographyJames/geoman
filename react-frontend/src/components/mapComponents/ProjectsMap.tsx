import { useEffect, useRef, useState } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import Overlay from "ol/Overlay";
import { Feature } from "ol";
import { Point } from "ol/geom";
import { fromLonLat } from "ol/proj";
import { useProjectsFilter, useHoveredProjectId } from "@/features/app/contexts/ProjectsFilterContext";

import { createMarkerStyles } from "@/components/mapComponents/markerStyles";
import { useMarkerAnimation } from "@/components/mapComponents/MarkerAnimation";
import type { MapBrowserEvent } from "ol";
import MapPopup from "@/components/mapComponents/MapPopup";
import type Project from "@/domain/project/entity";
import { Link, useSearch } from "@tanstack/react-router";

const HOVER_SCALE = 2;
const ANIM_STEPS = 8;

const activeStyles = createMarkerStyles({
  animationSteps: ANIM_STEPS,
  hoverScale: HOVER_SCALE,
  colour: "#2A81CB",
});
const archivedStyles = createMarkerStyles({
  animationSteps: ANIM_STEPS,
  hoverScale: HOVER_SCALE,
  colour: "#9CA3AF",
});

export default function ProjectsMap() {
  const { mapRef } = useMapContext();
  const { projects } = useProjectsFilter();
  const hoveredProjectId = useHoveredProjectId();
  const popupRef = useRef<HTMLDivElement>(null);
  const featuresRef = useRef<Map<number, Feature>>(new Map());
  const [popupContent, setPopupContent] = useState<{
    project: Project;
  } | null>(null);

  const { animateScale, hoveredFeatureRef, hoverScale } = useMarkerAnimation({
    activeStyles,
    archivedStyles,
    hoverScale: HOVER_SCALE,
    hoveredProjectId,
    featuresRef,
  });

  useEffect(() => {
    const map = mapRef.current;
    const popupEl = popupRef.current;
    if (!map || !popupEl) return;

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

    const vectorLayer = new VectorLayer({
      source: new VectorSource({ features }),
      style: (feature) =>
        feature.get("archived") ? archivedStyles[0] : activeStyles[0],
      updateWhileAnimating: true,
      updateWhileInteracting: true,
    });

    const overlay = new Overlay({
      element: popupEl,
      positioning: "bottom-center",
      offset: [0, -45],
      autoPan: true,
    });

    map.addLayer(vectorLayer);
    map.addOverlay(overlay);

    const handleClick = (e: MapBrowserEvent) => {
      const hit = map.forEachFeatureAtPixel(e.pixel, (feature) => feature, {
        layerFilter: (layer) => layer === vectorLayer,
      });

      if (hit && hit instanceof Feature) {
        const geom = hit.getGeometry() as Point;
        const project = projects.find((p) => p.id === hit.get("projectId"));
        if (project) {
          setPopupContent({ project });
          overlay.setPosition(geom.getCoordinates());
        }
      } else {
        overlay.setPosition(undefined);
        setPopupContent(null);
      }
    };

    const handlePointerMove = (e: MapBrowserEvent) => {
      const hit = map.forEachFeatureAtPixel(e.pixel, (f) => f, {
        layerFilter: (layer) => layer === vectorLayer,
      });

      const hoveredFeature = hoveredFeatureRef.current;

      if (hoveredFeature && hoveredFeature !== hit) {
        animateScale(hoveredFeature, hoverScale, 1);
        hoveredFeatureRef.current = null;
      }

      if (hit && hit instanceof Feature && hit !== hoveredFeature) {
        animateScale(hit, 1, hoverScale);
        hoveredFeatureRef.current = hit;
      }

      map.getTargetElement().style.cursor = hit ? "pointer" : "";
    };

    map.on("click", handleClick);
    map.on("pointermove", handlePointerMove);

    return () => {
      featuresRef.current.clear();
      map.removeLayer(vectorLayer);
      map.removeOverlay(overlay);
      map.un("click", handleClick);
      map.un("pointermove", handlePointerMove);
      const target = map.getTargetElement();
      if (target) target.style.cursor = "";
    };
  }, [mapRef, projects, animateScale, hoveredFeatureRef, hoverScale]);

  const closePopup = () => {
    mapRef.current
      ?.getOverlays()
      .getArray()
      .forEach((o) => o.setPosition(undefined));
    setPopupContent(null);
  };

  const { projects: projectsParam } = useSearch({ from: "/_app/" });
  const projectsArray = projectsParam ? projectsParam.split(",") : [];

  return (
    <MapPopup ref={popupRef} onClose={closePopup}>
      {popupContent && <PopupContent project={popupContent.project} projectsArray={projectsArray} onClose={closePopup} />}
    </MapPopup>
  );
}

function PopupContent({ project, projectsArray, onClose }: { project: Project; projectsArray: string[]; onClose: () => void }) {
  const newProjects = projectsArray.includes(project.slug)
    ? projectsArray.join(",")
    : [...projectsArray, project.slug].join(",");

  return (
    <>
      <Link
        from={"/"}
        search={{ projects: newProjects }}
        onClick={onClose}
        className="link font-semibold"
      >
        {project.name}
      </Link>
      <p>{`lat: ${project.latitude}`}</p>
      <p>{`long: ${project.longitude}`}</p>
      {project.centroidX != null && project.centroidY != null && project.crsSrid != null && (
        <>
          {project.crsName && <p className="text-base-content/70 text-xs">{project.crsName}</p>}
          {project.crsSrid && <p className="text-base-content/70 text-xs">EPSG:{project.crsSrid}</p>}
          <p>{`X: ${project.centroidX.toFixed(0)}`}</p>
          <p>{`Y: ${project.centroidY.toFixed(0)}`}</p>
        </>
      )}
    </>
  );
}
