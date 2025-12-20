import { createContext, useContext } from 'react'
import type { Map } from 'ol'

interface MapContextValue {
  containerRef: { current: HTMLDivElement | null };
  mapRef: { current: Map | null };
}

// Context to share the map container and instance with child routes
export const MapRefContext = createContext<MapContextValue | null>(null);

// Hook for child routes to access the map container and instance
export function useMapContext() {
  const context = useContext(MapRefContext);
  if (!context) {
    throw new Error('useMapContext must be used within AppLayout');
  }
  return context;
}
