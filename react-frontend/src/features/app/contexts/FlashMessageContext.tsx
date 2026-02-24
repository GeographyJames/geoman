import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from "react";

type FlashType = "success" | "error" | "warning" | "info";

interface FlashMessage {
  id: string;
  message: string;
  type: FlashType;
}

interface FlashContextValue {
  messages: FlashMessage[];
  addFlash: (message: string, type: FlashType) => void;
  removeFlash: (id: string) => void;
}

const FlashContext = createContext<FlashContextValue | null>(null);

export function FlashMessageProvider({ children }: { children: ReactNode }) {
  const [messages, setMessages] = useState<FlashMessage[]>([]);

  const addFlash = useCallback((message: string, type: FlashType) => {
    setMessages((prev) => [
      ...prev,
      { id: crypto.randomUUID(), message, type },
    ]);
  }, []);

  const removeFlash = useCallback((id: string) => {
    setMessages((prev) => prev.filter((m) => m.id !== id));
  }, []);

  const value = useMemo(
    () => ({ messages, addFlash, removeFlash }),
    [messages, addFlash, removeFlash],
  );

  return (
    <FlashContext.Provider value={value}>
      {children}
    </FlashContext.Provider>
  );
}

export function useFlash() {
  const context = useContext(FlashContext);
  if (!context) {
    throw new Error("useFlash must be used within FlashMessageProvider");
  }
  return context;
}
