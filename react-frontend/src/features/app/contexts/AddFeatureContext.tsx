import { createContext, useContext, useState, type ReactNode } from "react";
import type Project from "@/domain/project/entity";

interface AddFeatureContextValue {
  project: Project | null;
  preSelectedCollectionId: number | null;
  requestAddFeature: (project: Project, collectionId?: number) => void;
  clear: () => void;
}

const AddFeatureContext = createContext<AddFeatureContextValue | null>(null);

export function AddFeatureProvider({ children }: { children: ReactNode }) {
  const [project, setProject] = useState<Project | null>(null);
  const [preSelectedCollectionId, setPreSelectedCollectionId] = useState<number | null>(null);

  const requestAddFeature = (project: Project, collectionId?: number) => {
    setProject(project);
    setPreSelectedCollectionId(collectionId ?? null);
    const el = document.getElementById("add_site_feature");
    if (el instanceof HTMLDialogElement) {
      el.showModal();
    }
  };

  const clear = () => {
    setProject(null);
    setPreSelectedCollectionId(null);
  };

  return (
    <AddFeatureContext.Provider value={{ project, preSelectedCollectionId, requestAddFeature, clear }}>
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
