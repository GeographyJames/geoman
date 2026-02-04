import { lazy, Suspense } from "react";

import { Drawer } from "./Drawer";
import { SidebarProvider } from "@/features/app/contexts/SidebarContext";
import { SearchbarProvider } from "@/features/app/contexts/SearchbarContext";
import { ShowArchivedProjectsProvider } from "@/features/app/contexts/ShowArchivedProjectsContext";
import { FlashMessageProvider } from "@/features/app/contexts/FlashMessageContext";
import { DeleteFeatureProvider } from "@/features/app/contexts/DeleteFeatureContext";
import { EditFeatureProvider } from "@/features/app/contexts/EditFeatureContext";
import { DeleteFeatureForm } from "./forms/DeleteFeature";
import { EditFeatureForm } from "./forms/EditFeature";

const CreateProjectForm = lazy(() =>
  import("@/features/app/components/forms/CreateProject").then((module) => ({
    default: module.CreateProjectForm,
  })),
);

export const App = () => {
  return (
    <>
      <FlashMessageProvider>
        <SidebarProvider>
          <SearchbarProvider>
            <ShowArchivedProjectsProvider>
              <DeleteFeatureProvider>
                <EditFeatureProvider>
                  <Drawer />

                  <Suspense fallback={null}>
                    <CreateProjectForm />
                    <DeleteFeatureForm />
                    <EditFeatureForm />
                  </Suspense>
                </EditFeatureProvider>
              </DeleteFeatureProvider>
            </ShowArchivedProjectsProvider>
          </SearchbarProvider>
        </SidebarProvider>
      </FlashMessageProvider>
    </>
  );
};
