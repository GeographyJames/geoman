import { lazy, Suspense } from "react";

import { Drawer } from "./Drawer";
import { SidebarProvider } from "@/features/app/contexts/SidebarContext";
import { SearchbarProvider } from "@/features/app/contexts/SearchbarContext";
import { ShowArchivedProjectsProvider } from "@/features/app/contexts/ShowArchivedProjectsContext";

const CreateProjectForm = lazy(() =>
  import("@/features/app/components/forms/CreateProject").then((module) => ({
    default: module.CreateProjectForm,
  }))
);

export const App = () => {
  return (
    <>
      <SidebarProvider>
        <SearchbarProvider>
          <ShowArchivedProjectsProvider>
            <Drawer />

            <Suspense fallback={null}>
              <CreateProjectForm />
            </Suspense>
          </ShowArchivedProjectsProvider>
        </SearchbarProvider>
      </SidebarProvider>
    </>
  );
};
