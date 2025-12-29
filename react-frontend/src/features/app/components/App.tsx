import { lazy, Suspense } from "react";

import { Drawer } from "./Drawer";
import { SidebarProvider } from "@/features/app/contexts/SidebarContext";
import { SearchbarProvider } from "@/features/app/contexts/SearchbarContext";

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
          <Drawer />

          <Suspense fallback={null}>
            <CreateProjectForm />
          </Suspense>
        </SearchbarProvider>
      </SidebarProvider>
    </>
  );
};
