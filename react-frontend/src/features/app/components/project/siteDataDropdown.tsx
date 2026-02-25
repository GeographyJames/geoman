import type { Collection } from "@/hooks/api/useProjectCollections";
import ShowArchivedToggle from "../ShowArchivedToggle";
import { useState } from "react";
import { ProjectCollection } from "./ProjectCollection";
import { useProjectCollectionItems } from "@/hooks/api/useProjectCollectionItems";
import { useEditProjectCollection } from "../../contexts/EditProjectCollectionContext";
import { useDeleteProjectCollection } from "../../contexts/DeleteProjectCollectionContext";
import { Pencil, Trash2 } from "lucide-react";

export const SiteDataDropdown = ({
  collection,
  projectId,
}: {
  collection: Collection;
  projectId: number;
}) => {
  const [showArchived, setShowArchived] = useState<boolean>(false);
  const { requestEdit } = useEditProjectCollection();
  const { requestDelete } = useDeleteProjectCollection();

  const { data } = useProjectCollectionItems({
    projectId,
    collectionId: collection.id,
  });

  const collectionId = Number(collection.id);
  const hasFeatures = (data?.features.length ?? 0) > 0;

  return (
    <details className="collapse collapse-arrow bg-base-200 border-base-300 border">
      <summary className="flex justify-between collapse-title font-semibold after:start-5 after:end-auto p-2 pe-4 ps-12">
        <div>
          {collection.title}{" "}
          <span className="text-sm font-normal text-base-content/70">
            {`(${collection.geometry_type})`}
          </span>
        </div>
        <div className="font-normal text-xs flex flex-wrap gap-x-2 gap-y-1 items-center justify-end">
          {collection.project_id != null && (
            <>
              <button
                type="button"
                className="btn btn-ghost btn-xs px-1"
                title="Edit collection"
                onClick={(e) => {
                  e.preventDefault();
                  requestEdit({ id: collectionId, title: collection.title });
                }}
              >
                <Pencil size={12} />
              </button>
              <button
                type="button"
                className={`btn btn-ghost btn-xs px-1 ${hasFeatures ? "text-base-content/30" : "text-error"}`}
                title="Delete collection"
                disabled={hasFeatures}
                onClick={(e) => {
                  e.preventDefault();
                  requestDelete({ id: collectionId, title: collection.title });
                }}
              >
                <Trash2 size={12} />
              </button>
            </>
          )}
          <span className="text-base-content/70">{data && (() => { const count = showArchived ? data.features.length : data.features.filter((f) => f.properties.status !== "ARCHIVED").length; return `${count} feature${count !== 1 ? "s" : ""}`; })()}</span>
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
