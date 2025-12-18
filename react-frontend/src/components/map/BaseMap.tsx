import { useEffect, useRef } from "react";
import "ol/ol.css";

interface BaseMapProps {
  containerRef: { current: HTMLDivElement | null };
}

export default function BaseMap({ containerRef }: BaseMapProps) {
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
    />
  );
}