import { useRef, useEffect } from "react";
import type { Map } from "ol";
import { MapRefContext } from "@/features/app/contexts/MapRefContext";

import { CreateProjectForm } from "@/features/app/components/forms/CreateProject";

import { Drawer } from "./Drawer";
import { SidebarProvider } from "@/features/app/contexts/SidebarContext";
import { SearchbarProvider } from "@/features/app/contexts/SearchbarContext";
import { useAppSettings } from "@/hooks/api/useAppSettings";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";

export const App = () => {
  const containerRef = useRef<HTMLDivElement | null>(null);
  const mapRef = useRef<Map | null>(null);
  const appSettings = useAppSettings();
  const currentUser = useCurrentUser();
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

  if (appSettings.isError) {
    return <>Error loading application settings</>;
  }
  if (currentUser.isError) {
    return <>Error loading application user</>;
  }
  if (appSettings.data && currentUser.data)
    return (
      <>
        <SidebarProvider>
          <SearchbarProvider>
            <MapRefContext.Provider value={{ containerRef, mapRef }}>
              <Drawer />
            </MapRefContext.Provider>

            <CreateProjectForm
              currentUser={currentUser.data}
              technologies={appSettings.data.technologies}
            />
          </SearchbarProvider>
        </SidebarProvider>
      </>
    );
};
