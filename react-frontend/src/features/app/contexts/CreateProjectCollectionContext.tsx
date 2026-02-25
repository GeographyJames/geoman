import { createContext, useContext, useState, type ReactNode } from "react";
import type Project from "@/domain/project/entity";

const MODAL_ID = "create_project_collection";

interface CreateProjectCollectionContextValue {
  project: Project | null;
  requestCreateCollection: (project: Project) => void;
  clear: () => void;
}

const CreateProjectCollectionContext =
  createContext<CreateProjectCollectionContextValue | null>(null);

export function CreateProjectCollectionProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [project, setProject] = useState<Project | null>(null);

  const requestCreateCollection = (project: Project) => {
    setProject(project);
    const el = document.getElementById(MODAL_ID);
    if (el instanceof HTMLDialogElement) el.showModal();
  };

  const clear = () => setProject(null);

  return (
    <CreateProjectCollectionContext.Provider
      value={{ project, requestCreateCollection, clear }}
    >
      {children}
    </CreateProjectCollectionContext.Provider>
  );
}

export function useCreateProjectCollection() {
  const context = useContext(CreateProjectCollectionContext);
  if (!context) {
    throw new Error(
      "useCreateProjectCollection must be used within CreateProjectCollectionProvider",
    );
  }
  return context;
}
