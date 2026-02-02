import { lazy, Suspense } from "react";

import { Drawer } from "./Drawer";
import { SidebarProvider } from "@/features/app/contexts/SidebarContext";
import { SearchbarProvider } from "@/features/app/contexts/SearchbarContext";
import { ShowArchivedProjectsProvider } from "@/features/app/contexts/ShowArchivedProjectsContext";
import { FlashMessageProvider } from "@/features/app/contexts/FlashMessageContext";

const CreateProjectForm = lazy(() =>
  import("@/features/app/components/forms/CreateProject").then((module) => ({
    default: module.CreateProjectForm,
  }))
);

export const App = () => {
  return (
    <>
      <FlashMessageProvider>
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
      </FlashMessageProvider>
    </>
  );
};
