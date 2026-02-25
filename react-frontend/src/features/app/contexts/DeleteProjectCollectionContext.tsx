import { createContext, useContext, useState, type ReactNode } from "react";
import type { CollectionForDelete } from "@/features/admin/sections/collections/DeleteCollectionForm";

const MODAL_ID = "delete_project_collection";

interface DeleteProjectCollectionContextValue {
  collection: CollectionForDelete | null;
  requestDelete: (collection: CollectionForDelete) => void;
  clear: () => void;
}

const DeleteProjectCollectionContext =
  createContext<DeleteProjectCollectionContextValue | null>(null);

export function DeleteProjectCollectionProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [collection, setCollection] = useState<CollectionForDelete | null>(
    null,
  );

  const requestDelete = (collection: CollectionForDelete) => {
    setCollection(collection);
    const el = document.getElementById(MODAL_ID);
    if (el instanceof HTMLDialogElement) el.showModal();
  };

  const clear = () => setCollection(null);

  return (
    <DeleteProjectCollectionContext.Provider
      value={{ collection, requestDelete, clear }}
    >
      {children}
    </DeleteProjectCollectionContext.Provider>
  );
}

export function useDeleteProjectCollection() {
  const context = useContext(DeleteProjectCollectionContext);
  if (!context) {
    throw new Error(
      "useDeleteProjectCollection must be used within DeleteProjectCollectionProvider",
    );
  }
  return context;
}
