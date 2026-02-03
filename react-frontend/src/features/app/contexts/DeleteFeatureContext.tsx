import { createContext, useContext, useState, type ReactNode } from "react";
import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";

interface DeleteFeatureContextValue {
  feature: ProjectCollectionItem | null;
  requestDelete: (feature: ProjectCollectionItem) => void;
  clear: () => void;
}

const DeleteFeatureContext = createContext<DeleteFeatureContextValue | null>(
  null,
);

export function DeleteFeatureProvider({ children }: { children: ReactNode }) {
  const [feature, setFeature] = useState<ProjectCollectionItem | null>(null);

  const requestDelete = (feature: ProjectCollectionItem) => {
    setFeature(feature);
    const el = document.getElementById("delete_feature");
    if (el instanceof HTMLDialogElement) {
      el.show();
    }
  };

  const clear = () => setFeature(null);

  return (
    <DeleteFeatureContext.Provider value={{ feature, requestDelete, clear }}>
      {children}
    </DeleteFeatureContext.Provider>
  );
}

export function useDeleteFeature() {
  const context = useContext(DeleteFeatureContext);
  if (!context) {
    throw new Error(
      "useDeleteFeature must be used within DeleteFeatureProvider",
    );
  }
  return context;
}
