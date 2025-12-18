import { Outlet, createFileRoute } from "@tanstack/react-router";
import BaseMap from "@/components/map/BaseMap";
import { useRef, useEffect } from "react";
import type { Map } from "ol";
import { MapRefContext } from "@/contexts/MapRefContext";
import { OverviewSidebarProvider } from "@/contexts/SidebarContext";
import NavBar from "@/components/nav/NavBar";
import { SearchBar } from "@/components/search/SearchBar";

export const Route = createFileRoute("/_app")({
  component: AppLayout,
});

function AppLayout() {
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

  return (
    <MapRefContext.Provider value={{ containerRef, mapRef }}>
      <OverviewSidebarProvider>
        <BaseMap containerRef={containerRef} />
        <Outlet />
        <div className="flex flex-wrap absolute top-0 m-4 gap-2">
          <SearchBar />
          <NavBar />
        </div>
      </OverviewSidebarProvider>
    </MapRefContext.Provider>
  );
}
