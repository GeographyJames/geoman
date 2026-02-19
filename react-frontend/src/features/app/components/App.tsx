import { lazy, Suspense } from "react";

import { Drawer } from "./Drawer";
import { SidebarProvider } from "@/features/app/contexts/SidebarContext";
import { SearchbarProvider } from "@/features/app/contexts/SearchbarContext";
import { FlashMessageProvider } from "@/features/app/contexts/FlashMessageContext";
import { DeleteFeatureProvider } from "@/features/app/contexts/DeleteFeatureContext";
import { EditFeatureProvider } from "@/features/app/contexts/EditFeatureContext";
import { DeleteFeatureForm } from "./forms/DeleteFeature";
import { EditFeatureForm } from "./forms/EditFeature";
import { EditProjectProvider } from "../contexts/EditProjectContext";
import { EditProjectForm } from "./forms/EditProject";
import { DeleteProjectProvider } from "../contexts/DeleteProjectContext";
import { DeleteProjectForm } from "./forms/DeleteProject";
import { AddFeatureProvider } from "../contexts/AddFeatureContext";
import { AddSiteFeatureForm } from "./forms/AddSiteFeature";

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
            <DeleteFeatureProvider>
              <EditFeatureProvider>
                <EditProjectProvider>
                  <DeleteProjectProvider>
                    <AddFeatureProvider>
                      <Drawer />
                      <Suspense fallback={null}>
                        <CreateProjectForm />
                        <DeleteFeatureForm />
                        <EditFeatureForm />
                        <EditProjectForm />
                        <AddSiteFeatureForm />
                        <DeleteProjectForm />
                      </Suspense>
                    </AddFeatureProvider>
                  </DeleteProjectProvider>
                </EditProjectProvider>
              </EditFeatureProvider>
            </DeleteFeatureProvider>
          </SearchbarProvider>
        </SidebarProvider>
      </FlashMessageProvider>
    </>
  );
};
