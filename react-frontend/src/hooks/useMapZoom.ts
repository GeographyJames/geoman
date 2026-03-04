import { useEffect, useState } from "react";
import { useMapContext } from "@/features/app/contexts/MapRefContext";

/** Returns the current map zoom level, updating as the user pans/zooms. */
export function useMapZoom(): number | undefined {
  const { mapRef } = useMapContext();
  const [zoom, setZoom] = useState<number | undefined>(
    () => mapRef.current?.getView().getZoom(),
  );

  useEffect(() => {
    const map = mapRef.current;
    if (!map) return;

    const onMoveEnd = () => setZoom(map.getView().getZoom());
    map.on("moveend", onMoveEnd);

    return () => {
      map.un("moveend", onMoveEnd);
    };
  }, [mapRef]);

  return zoom;
}
