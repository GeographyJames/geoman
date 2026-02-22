import { createContext, useContext, useState, type ReactNode } from "react";
interface SearchBarContextValue {
  isOpen: boolean;
  setIsOpen: (open: boolean) => void;
  toggleSearchbar: () => void;
}

const SearchbarContext = createContext<SearchBarContextValue | null>(null);

// Stable setter — consumers NEVER re-render when isOpen changes
const SearchbarActionsContext = createContext<(open: boolean) => void>(() => {});

export function SearchbarProvider({ children }: { children: ReactNode }) {
  const [isOpen, setIsOpen] = useState(false);
  const toggleSearchbar = () => setIsOpen((prev) => !prev);
  return (
    <SearchbarActionsContext.Provider value={setIsOpen}>
      <SearchbarContext.Provider value={{ isOpen, setIsOpen, toggleSearchbar }}>
        {children}
      </SearchbarContext.Provider>
    </SearchbarActionsContext.Provider>
  );
}

export function useSearchbar() {
  const context = useContext(SearchbarContext);
  if (!context) {
    throw new Error("useSidebar must be used within SidebarProvider");
  }
  return context;
}

export function useSearchbarSetOpen() {
  return useContext(SearchbarActionsContext);
}
