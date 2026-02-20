import { forwardRef, type ReactNode } from "react";

interface MapPopupProps {
  children?: ReactNode;
  onClose: () => void;
}

const MapPopup = forwardRef<HTMLDivElement, MapPopupProps>(
  ({ children, onClose }, ref) => {
    return (
      <div ref={ref}>
        <div className="bg-base-100 rounded-lg shadow-lg px-3 py-2 text-sm relative">
          <button
            onClick={onClose}
            className="absolute top-1 right-1 text-base-content/50 hover:text-base-content text-sm leading-none cursor-pointer"
          >
            ✕
          </button>
          <div className="pr-4">{children}</div>
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 top-full w-0 h-0 border-l-[8px] border-l-transparent border-r-[8px] border-r-transparent border-t-[8px] border-t-base-100" />
      </div>
    );
  },
);

MapPopup.displayName = "MapPopup";

export default MapPopup;
