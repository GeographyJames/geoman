import { Outlet, createFileRoute, Link } from "@tanstack/react-router";
import BaseMap from "@/components/map/BaseMap";
import { useRef, useEffect, useState } from "react";
import type { Map } from "ol";
import { MapRefContext } from "@/contexts/MapRefContext";

import { SearchBar } from "@/components/search/SearchBar";
import { MdAdminPanelSettings } from "react-icons/md";
import { HiInformationCircle } from "react-icons/hi";
import { CreateProjectForm } from "@/components/forms/CreateProject";

const AppLayout = () => {
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
    <>
      <MapRefContext.Provider value={{ containerRef, mapRef }}>
        <div className="drawer h-full">
          <input
            id="my-drawer-1"
            type="checkbox"
            className="drawer-toggle"
            checked={sidebarOpen}
            onChange={(e) => setSidebarOpen(e.target.checked)}
          />
          <div className="drawer-content h-full">
            <BaseMap
              containerRef={containerRef}
              onMapClick={() => setSearchOpen(false)}
            />
            <div className="flex flex-col gap-2 absolute top-0 m-4">
              <div className="flex flex-wrap  gap-2">
                <SearchBar
                  setSidebarOpen={setSidebarOpen}
                  searchOpen={searchOpen}
                  setSearchOpen={setSearchOpen}
                />
              </div>
              <Outlet />
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
                <Link to="/admin" className="flex gap-2 items-center">
                  <MdAdminPanelSettings size={18} />
                  Admin
                </Link>
              </li>
              <li>
                <Link to="/about" className="flex gap-2 items-center">
                  <HiInformationCircle size={18} />
                  About
                </Link>
              </li>
            </ul>
          </div>
        </div>
      </MapRefContext.Provider>
      <CreateProjectForm />
    </>
  );
};

export const Route = createFileRoute("/_app")({
  component: AppLayout,
});
