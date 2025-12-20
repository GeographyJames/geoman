import {
  ShowArchivedBoundariesContext,
  ShowArchivedFiguresContext,
  ShowArchivedLayoutsContext,
  ShowArchivedProjectsContext,
  ShowArchivedSearchAreasContext,
  type ShowArchivedType,
} from "@/features/app/contexts/showArchivedContext";
import React, { useContext } from "react";

export function ShowArchivedProjectsToggle() {
  const context = useContext(ShowArchivedProjectsContext);

  return <ShowArchivedToggle showArchivedContext={context} />;
}

export function ShowArchivedBoundariesToggle() {
  const context = useContext(ShowArchivedBoundariesContext);
  return <ShowArchivedToggle showArchivedContext={context} />;
}

export function ShowArchivedLayoutsToggle() {
  const context = useContext(ShowArchivedLayoutsContext);
  return <ShowArchivedToggle showArchivedContext={context} />;
}

export function ShowArchivedSearchAreasToggle() {
  const context = useContext(ShowArchivedSearchAreasContext);
  return <ShowArchivedToggle showArchivedContext={context} />;
}
export function ShowArchivedFiguresToggle() {
  const context = useContext(ShowArchivedFiguresContext);
  return <ShowArchivedToggle showArchivedContext={context} />;
}

function ShowArchivedToggle({
  showArchivedContext,
}: {
  showArchivedContext: ShowArchivedType;
}) {
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    showArchivedContext.dispatch(event.target.checked);
  };
  return (
    <div className="flex content-center">
      <label className="label cursor-pointer p-0">
        <span className="label-text text-xs">show archived</span>
        <input
          onChange={(e) => handleChange(e)}
          type="checkbox"
          className="toggle toggle-xs"
          checked={showArchivedContext.showArchived}
        />
      </label>
    </div>
  );
}
