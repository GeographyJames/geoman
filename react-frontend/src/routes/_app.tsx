import { Outlet, createFileRoute } from "@tanstack/react-router";
import BaseMap from "@/components/map/BaseMap";
import { useRef, useEffect, useState } from "react";
import type { Map } from "ol";
import { MapRefContext } from "@/contexts/MapRefContext";
import { OverviewSidebarProvider } from "@/contexts/SidebarContext";
import { SearchBar } from "@/components/search/SearchBar";

export const Route = createFileRoute("/_app")({
  component: AppLayout,
});

function AppLayout() {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const mapRef = useRef<Map | null>(null);
  const [sidebarOpen, setSidebarOpen] = useState<boolean>(false);
  const [searchOpen, setSearchOpen] = useState<boolean>(false);

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
        <div className="drawer h-full">
          <input
            id="my-drawer-1"
            type="checkbox"
            className="drawer-toggle"
            checked={sidebarOpen}
            onChange={(e) => setSidebarOpen(e.target.checked)}
          />
          <div className="drawer-content h-full">
            <BaseMap containerRef={containerRef} onMapClick={() => setSearchOpen(false)} />
            <Outlet />
            <div className="flex flex-wrap absolute top-0 m-4 gap-2">
              <SearchBar
                setSidebarOpen={setSidebarOpen}
                searchOpen={searchOpen}
                setSearchOpen={setSearchOpen}
              />
            </div>
          </div>
          <div className="drawer-side">
            <label
              htmlFor="my-drawer-1"
              aria-label="close sidebar"
              className="drawer-overlay"
            ></label>
            <ul className="menu bg-base-200 min-h-full w-80 p-4">
              <li>
                <a>Sidebar Item 1</a>
              </li>
              <li>
                <a>Sidebar Item 2</a>
              </li>
            </ul>
          </div>
        </div>
      </OverviewSidebarProvider>
    </MapRefContext.Provider>
  );
}
