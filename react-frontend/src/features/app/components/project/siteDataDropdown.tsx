import type { Collection } from "@/hooks/api/useProjectCollections";
import ShowArchivedToggle from "../ShowArchivedToggle";
import { useState } from "react";
import { ProjectCollection } from "./ProjectCollection";

export const SiteDataDropdown = ({
  collection,
  projectId,
}: {
  collection: Collection;
  projectId: number;
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
        <ProjectCollection collectionId={collection.id} projectId={projectId} />
      </div>
    </details>
  );
};
