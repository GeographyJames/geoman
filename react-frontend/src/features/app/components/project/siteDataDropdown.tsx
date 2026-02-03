import type { Collection } from "@/hooks/api/useProjectCollections";
import ShowArchivedToggle from "../ShowArchivedToggle";
import { useState } from "react";
import { ProjectCollection } from "./ProjectCollection";
import { useProjectCollectionItems } from "@/hooks/api/useProjectCollectionItems";

export const SiteDataDropdown = ({
  collection,
  projectId,
}: {
  collection: Collection;
  projectId: number;
}) => {
  const [showArchived, setShowArchived] = useState<boolean>(false);

  const { data } = useProjectCollectionItems({
    projectId,
    collectionId: collection.id,
  });
  return (
    <details className="collapse collapse-arrow bg-base-200 border-base-300 border">
      <summary className="flex justify-between collapse-title font-semibold after:start-5 after:end-auto p-2 pe-4 ps-12">
        <div>
          {collection.title}{" "}
          <span className="text-sm font-normal text-base-content/70">
            {`(${collection.geometry_type})`}
          </span>
        </div>
        <div className="font-normal text-xs flex gap-2 items-center">
          <span className="text-base-content/70">{`${data && data.features.length} features`}</span>
          <ShowArchivedToggle
            setShowArchived={setShowArchived}
            showArchived={showArchived}
          />
        </div>
      </summary>
      <div className="collapse-content text-sm pb-0">
        {data && <ProjectCollection data={data} showArchived={showArchived} />}
      </div>
    </details>
  );
};
