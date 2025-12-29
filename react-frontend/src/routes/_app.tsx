import { App } from "@/features/app/components/App";
import { createFileRoute, Navigate, Outlet } from "@tanstack/react-router";
import { useAuth } from "@clerk/clerk-react";
import { useRef, useEffect } from "react";
import type { Map } from "ol";
import { MapRefContext } from "@/features/app/contexts/MapRefContext";

type MapParams = {
  projects?: string;
};

const AppLayout = () => {
  const { isSignedIn, isLoaded } = useAuth();
  const isDemoMode = __RUN_ENVIRONMENT__ === "development";
  const containerRef = useRef<HTMLDivElement | null>(null);
  const mapRef = useRef<Map | null>(null);

  // Cleanup: destroy map when leaving the _app layout
  useEffect(() => {
    return () => {
      if (mapRef.current) {
        console.log("Destroying map (leaving _app layout)");
        mapRef.current.setTarget(undefined);
        mapRef.current.dispose();
        mapRef.current = null;
      }
    };
  }, []);

  // Wait for Clerk to load
  if (!isLoaded) {
    return null;
  }

  // Redirect if not authenticated (unless demo mode)
  if (!isSignedIn && !isDemoMode) {
    return <Navigate to="/about" />;
  }

  return (
    <MapRefContext.Provider value={{ containerRef, mapRef }}>
      <Outlet />
    </MapRefContext.Provider>
  );
};

export const Route = createFileRoute("/_app")({
  validateSearch: (search: Record<string, unknown>): MapParams => {
    return {
      projects: search.projects as string,
    };
  },
  component: AppLayout,
});
