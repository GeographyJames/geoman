import { createContext, useContext, useState, type ReactNode } from "react";
import type { CollectionForEdit } from "@/features/admin/sections/collections/EditCollectionForm";

const MODAL_ID = "edit_project_collection";

interface EditProjectCollectionContextValue {
  collection: CollectionForEdit | null;
  requestEdit: (collection: CollectionForEdit) => void;
  clear: () => void;
}

const EditProjectCollectionContext =
  createContext<EditProjectCollectionContextValue | null>(null);

export function EditProjectCollectionProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [collection, setCollection] = useState<CollectionForEdit | null>(null);

  const requestEdit = (collection: CollectionForEdit) => {
    setCollection(collection);
    const el = document.getElementById(MODAL_ID);
    if (el instanceof HTMLDialogElement) el.showModal();
  };

  const clear = () => setCollection(null);

  return (
    <EditProjectCollectionContext.Provider
      value={{ collection, requestEdit, clear }}
    >
      {children}
    </EditProjectCollectionContext.Provider>
  );
}

export function useEditProjectCollection() {
  const context = useContext(EditProjectCollectionContext);
  if (!context) {
    throw new Error(
      "useEditProjectCollection must be used within EditProjectCollectionProvider",
    );
  }
  return context;
}
