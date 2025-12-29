import { useRef, useEffect, lazy, Suspense } from "react";
import type { Map } from "ol";
import { MapRefContext } from "@/features/app/contexts/MapRefContext";

import { Drawer } from "./Drawer";
import { SidebarProvider } from "@/features/app/contexts/SidebarContext";
import { SearchbarProvider } from "@/features/app/contexts/SearchbarContext";

const CreateProjectForm = lazy(() =>
  import("@/features/app/components/forms/CreateProject").then((module) => ({
    default: module.CreateProjectForm,
  }))
);

export const App = () => {
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
    <>
      <SidebarProvider>
        <SearchbarProvider>
          <MapRefContext.Provider value={{ containerRef, mapRef }}>
            <Drawer />
          </MapRefContext.Provider>

          <Suspense fallback={null}>
            <CreateProjectForm />
          </Suspense>
        </SearchbarProvider>
      </SidebarProvider>
    </>
  );
};
