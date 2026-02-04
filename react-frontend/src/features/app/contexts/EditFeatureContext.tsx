import { createContext, useContext, useState, type ReactNode } from "react";
import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";

interface EditFeatureContextValue {
  feature: ProjectCollectionItem | null;
  requestEdit: (feature: ProjectCollectionItem) => void;
  clear: () => void;
}

const EditFeatureContext = createContext<EditFeatureContextValue | null>(null);

export function EditFeatureProvider({ children }: { children: ReactNode }) {
  const [feature, setFeature] = useState<ProjectCollectionItem | null>(null);

  const requestEdit = (feature: ProjectCollectionItem) => {
    setFeature(feature);
    const el = document.getElementById("edit_feature");
    if (el instanceof HTMLDialogElement) {
      el.show();
    }
  };

  const clear = () => setFeature(null);

  return (
    <EditFeatureContext.Provider value={{ feature, requestEdit, clear }}>
      {children}
    </EditFeatureContext.Provider>
  );
}

export function useEditFeature() {
  const context = useContext(EditFeatureContext);
  if (!context) {
    throw new Error(
      "useEditFeature must be used within EditFeatureProvider",
    );
  }
  return context;
}
