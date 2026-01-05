import { createContext, useContext, useState, type ReactNode } from "react";

interface ShowArchivedProjectsContextType {
  showArchivedProjects: boolean;
  setShowArchivedProjects: (value: boolean) => void;
}

const ShowArchivedProjectsContext = createContext<
  ShowArchivedProjectsContextType | undefined
>(undefined);

export function ShowArchivedProjectsProvider({
  children,
}: {
  children: ReactNode;
}) {
  const [showArchivedProjects, setShowArchivedProjects] =
    useState<boolean>(false);

  return (
    <ShowArchivedProjectsContext.Provider
      value={{ showArchivedProjects, setShowArchivedProjects }}
    >
      {children}
    </ShowArchivedProjectsContext.Provider>
  );
}

export function useShowArchivedProjects() {
  const context = useContext(ShowArchivedProjectsContext);
  if (context === undefined) {
    throw new Error(
      "useShowArchivedProjects must be used within a ShowArchivedProjectsProvider"
    );
  }
  return context;
}
