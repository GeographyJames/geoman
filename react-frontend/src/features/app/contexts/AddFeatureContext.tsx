import { createContext, useContext, useState, type ReactNode } from "react";
import type Project from "@/domain/project/entity";

interface AddFeatureContextValue {
  project: Project | null;
  requestAddFeature: (project: Project) => void;
  clear: () => void;
}

const AddFeatureContext = createContext<AddFeatureContextValue | null>(null);

export function AddFeatureProvider({ children }: { children: ReactNode }) {
  const [project, setProject] = useState<Project | null>(null);

  const requestAddFeature = (project: Project) => {
    setProject(project);
    const el = document.getElementById("add_site_feature");
    if (el instanceof HTMLDialogElement) {
      el.showModal();
    }
  };

  const clear = () => setProject(null);

  return (
    <AddFeatureContext.Provider value={{ project, requestAddFeature, clear }}>
      {children}
    </AddFeatureContext.Provider>
  );
}

export function useAddFeature() {
  const context = useContext(AddFeatureContext);
  if (!context) {
    throw new Error("useAddFeature must be used within AddFeatureProvider");
  }
  return context;
}
