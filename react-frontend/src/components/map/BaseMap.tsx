import { useEffect, useRef } from "react";
import "ol/ol.css";

interface BaseMapProps {
  containerRef: { current: HTMLDivElement | null };
  onMapClick?: () => void;
}

export default function BaseMap({ containerRef, onMapClick }: BaseMapProps) {
  const divRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (divRef.current) {
      containerRef.current = divRef.current;
    }

    return () => {
      containerRef.current = null;
    };
  }, [containerRef]);

  return (
    <div
      ref={divRef}
      className="w-full h-full"
      style={{ backgroundColor: '#f0f0f0' }}
      onClick={onMapClick}
    />
  );
}