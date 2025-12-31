import type { Collection } from "@/hooks/api/useProjectCollections";
import ShowArchivedToggle from "../ShowArchivedToggle";
import { useState } from "react";

export const SiteDataDropdown = ({
  collection,
}: {
  collection: Collection;
}) => {
  const [showArchived, setShowArchived] = useState<boolean>(false);
  return (
    <details className="collapse collapse-arrow bg-base-200 border-base-300 border">
      <summary className="flex justify-between collapse-title font-semibold after:start-5 after:end-auto p-2 pe-4 ps-12">
        {collection.title}
        <ShowArchivedToggle
          setShowArchived={setShowArchived}
          showArchived={showArchived}
        />
      </summary>
      <div className="collapse-content text-sm">
        Click the "Sign Up" button in the top right corner and follow the
        registration process.
      </div>
    </details>
  );
};
