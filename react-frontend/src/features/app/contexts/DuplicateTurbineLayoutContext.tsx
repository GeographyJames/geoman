import { createContext, useContext, useState, type ReactNode } from "react";
import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";

interface DuplicateTurbineLayoutContextValue {
    feature: ProjectCollectionItem | null;
    requestDuplicate: (feature: ProjectCollectionItem) => void;
    clear: () => void;
}

const DuplicateTurbineLayoutContext = createContext<DuplicateTurbineLayoutContextValue | null>(null);

export function DuplicateTurbineLayoutProvider({ children }: { children: ReactNode }) {
    const [feature, setFeature] = useState<ProjectCollectionItem | null>(null);

    const requestDuplicate = (feature: ProjectCollectionItem) => {
        setFeature(feature);
        const el = document.getElementById("duplicate_turbine_layout");
        if (el instanceof HTMLDialogElement) {
            el.showModal();
        }
    };

    const clear = () => setFeature(null);

    return (
        <DuplicateTurbineLayoutContext.Provider value={{ feature, requestDuplicate, clear }}>
            {children}
        </DuplicateTurbineLayoutContext.Provider>
    );
}

export function useDuplicateTurbineLayoutContext() {
    const context = useContext(DuplicateTurbineLayoutContext);
    if (!context) {
        throw new Error("useDuplicateTurbineLayoutContext must be used within DuplicateTurbineLayoutProvider");
    }
    return context;
}
