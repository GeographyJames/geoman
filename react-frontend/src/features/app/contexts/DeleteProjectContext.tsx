import { createContext, useContext, useState, type ReactNode } from "react";
import type Project from "@/domain/project/entity";

interface DeleteProjectContextValue {
  project: Project | null;
  requestDelete: (project: Project) => void;
  clear: () => void;
}

const DeleteProjectContext = createContext<DeleteProjectContextValue | null>(
  null,
);

export function DeleteProjectProvider({ children }: { children: ReactNode }) {
  const [project, setProject] = useState<Project | null>(null);

  const requestDelete = (project: Project) => {
    setProject(project);
    const el = document.getElementById("delete_project");
    if (el instanceof HTMLDialogElement) {
      el.showModal();
    }
  };

  const clear = () => setProject(null);

  return (
    <DeleteProjectContext.Provider value={{ project, requestDelete, clear }}>
      {children}
    </DeleteProjectContext.Provider>
  );
}

export function useDeleteProject() {
  const context = useContext(DeleteProjectContext);
  if (!context) {
    throw new Error(
      "useDeleteProject must be used within DeleteProjectProvider",
    );
  }
  return context;
}
