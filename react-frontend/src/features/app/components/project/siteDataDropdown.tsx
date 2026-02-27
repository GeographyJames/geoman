import type { Collection } from "@/hooks/api/useProjectCollections";
import ShowArchivedToggle from "../ShowArchivedToggle";
import { useState } from "react";
import { ProjectCollection } from "./ProjectCollection";
import { useProjectCollectionItems } from "@/hooks/api/useProjectCollectionItems";
import { useEditProjectCollection } from "../../contexts/EditProjectCollectionContext";
import { useDeleteProjectCollection } from "../../contexts/DeleteProjectCollectionContext";
import { useAddFeature } from "../../contexts/AddFeatureContext";
import { CreateIconButton } from "@/components/Buttons";
import { Download, Pencil, Trash2 } from "lucide-react";
import type Project from "@/domain/project/entity";

export const SiteDataDropdown = ({
  collection,
  project,
}: {
  collection: Collection;
  project: Project;
}) => {
  const projectId = project.id;
  const [showArchived, setShowArchived] = useState<boolean>(false);
  const { requestEdit } = useEditProjectCollection();
  const { requestDelete } = useDeleteProjectCollection();
  const { requestAddFeature } = useAddFeature();

  const { data } = useProjectCollectionItems({
    projectId,
    collectionId: collection.id,
  });

  const collectionId = Number(collection.id);
  const hasFeatures = (data?.features.length ?? 0) > 0;

  return (
    <details className="collapse collapse-arrow  bg-base-100 rounded-sm shadow-xl">
      <summary className="flex justify-between collapse-title font-semibold after:start-5 after:end-auto p-1 pe-2 ps-12 ">
        <div>
          {collection.title}{" "}
          <span className="text-sm font-normal text-base-content/70">
            {`(${collection.geometry_type})`}
          </span>
        </div>
        <div className="font-normal text-xs flex items-center justify-end gap-x-2">
          {collection.project_id != null && (
            <div className="flex gap-x-1">
              <button
                type="button"
                className="btn btn-ghost btn-xs px-1 hover:bg-base-300"
                title="Edit collection"
                onClick={(e) => {
                  e.preventDefault();
                  requestEdit({
                    id: collectionId,
                    title: collection.title,
                    description: collection.description,
                  });
                }}
              >
                <Pencil size={12} />
              </button>
              <button
                type="button"
                className={`btn btn-ghost btn-xs px-1 hover:bg-base-300 ${hasFeatures ? "text-base-content/30" : "text-error"}`}
                title="Delete collection"
                disabled={hasFeatures}
                onClick={(e) => {
                  e.preventDefault();
                  requestDelete({ id: collectionId, title: collection.title });
                }}
              >
                <Trash2 size={12} />
              </button>
            </div>
          )}
          <button
            type="button"
            className="btn btn-outline btn-xs btn-square px-1"
            title="Download collection"
            onClick={(e) => e.preventDefault()}
          >
            <Download size={12} />
          </button>
          <CreateIconButton
            title="Add feature"
            onClick={() => requestAddFeature(project, collectionId)}
            className=""
          />
        </div>
      </summary>
      <div className="collapse-content text-sm pb-1 mt-1">
        <div className="flex justify-between items-end mb-2">
          {collection.description ? (
            <p className="text-xs text-base-content/60">
              {collection.description}
            </p>
          ) : (
            <span />
          )}
          <div className="flex items-center gap-x-1 flex-wrap justify-end">
            <span className="text-base-content/70 text-xs w-14 text-right">
              {data &&
                (() => {
                  const count = showArchived
                    ? data.features.length
                    : data.features.filter(
                        (f) => f.properties.status !== "ARCHIVED",
                      ).length;
                  return `${count} feature${count !== 1 ? "s" : ""}`;
                })()}
            </span>
            <ShowArchivedToggle
              setShowArchived={setShowArchived}
              showArchived={showArchived}
            />
          </div>
        </div>
        {data && <ProjectCollection data={data} showArchived={showArchived} />}
      </div>
    </details>
  );
};
