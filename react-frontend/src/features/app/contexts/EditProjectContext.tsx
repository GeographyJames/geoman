import { createContext, useContext, useState, type ReactNode } from "react";
import type Project from "@/domain/project/entity";

interface EditProjectContextValue {
  project: Project | null;
  requestEdit: (project: Project) => void;
  clear: () => void;
}

const EditProjectContext = createContext<EditProjectContextValue | null>(null);

export function EditProjectProvider({ children }: { children: ReactNode }) {
  const [project, setProject] = useState<Project | null>(null);

  const requestEdit = (project: Project) => {
    setProject(project);
    const el = document.getElementById("edit_project");
    if (el instanceof HTMLDialogElement) {
      el.show();
    }
  };

  const clear = () => setProject(null);

  return (
    <EditProjectContext.Provider value={{ project, requestEdit, clear }}>
      {children}
    </EditProjectContext.Provider>
  );
}

export function useEditProject() {
  const context = useContext(EditProjectContext);
  if (!context) {
    throw new Error(
      "useEditProject must be used within EditProjectProvider",
    );
  }
  return context;
}
