import { createContext, useCallback, useContext, useState, type ReactNode } from "react";
interface SidebarContextValue {
  isOpen: boolean;
  setIsOpen: (open: boolean) => void;
  toggleSidebar: () => void;
}

const SidebarContext = createContext<SidebarContextValue | null>(null);

// Stable toggle — consumers NEVER re-render when isOpen changes
const SidebarActionsContext = createContext<() => void>(() => {});

export function SidebarProvider({ children }: { children: ReactNode }) {
  const [isOpen, setIsOpen] = useState(false);
  const toggleSidebar = useCallback(() => setIsOpen((prev) => !prev), []);
  return (
    <SidebarActionsContext.Provider value={toggleSidebar}>
      <SidebarContext.Provider value={{ isOpen, setIsOpen, toggleSidebar }}>
        {children}
      </SidebarContext.Provider>
    </SidebarActionsContext.Provider>
  );
}

export function useSidebar() {
  const context = useContext(SidebarContext);
  if (!context) {
    throw new Error("useSidebar must be used within SidebarProvider");
  }
  return context;
}

export function useSidebarToggle() {
  return useContext(SidebarActionsContext);
}
