import type { Collection } from "@/hooks/api/useProjectCollections";
import ShowArchivedToggle from "../ShowArchivedToggle";
import { useEffect, useRef, useState } from "react";
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

  const [visibilityMap, setVisibilityMap] = useState<Record<number, boolean>>(
    {},
  );

  useEffect(() => {
    if (data) {
      setVisibilityMap((prev) => {
        const next: Record<number, boolean> = { ...prev };
        let changed = false;
        for (const f of data.features) {
          if (!(f.id in next)) {
            next[f.id] = f.properties.is_primary;
            changed = true;
          }
        }
        return changed ? next : prev;
      });
    }
  }, [data]);

  const filteredFeatures = showArchived
    ? (data?.features ?? [])
    : (data?.features ?? []).filter((f) => f.properties.status !== "ARCHIVED");

  const allVisible =
    filteredFeatures.length > 0 &&
    filteredFeatures.every((f) => visibilityMap[f.id]);
  const someVisible = filteredFeatures.some((f) => visibilityMap[f.id]);

  const toggleAll = () => {
    const newVal = !allVisible;
    setVisibilityMap((prev) => ({
      ...prev,
      ...Object.fromEntries(filteredFeatures.map((f) => [f.id, newVal])),
    }));
  };

  const checkboxRef = useRef<HTMLInputElement>(null);
  useEffect(() => {
    if (checkboxRef.current) {
      checkboxRef.current.indeterminate = someVisible && !allVisible;
    }
  }, [allVisible, someVisible]);

  const collectionId = Number(collection.id);
  const hasFeatures = (data?.features.length ?? 0) > 0;

  return (
    <details className="collapse collapse-arrow  bg-base-100 rounded-sm shadow-xl">
      <summary className="flex justify-between collapse-title font-semibold after:start-3 after:end-auto p-1 pe-2 ps-8 gap-2">
        <div className="flex items-center gap-2">
          <input
            ref={checkboxRef}
            type="checkbox"
            className="checkbox checkbox-sm bg-base-100"
            checked={allVisible}
            onChange={toggleAll}
            onClick={(e) => e.stopPropagation()}
          />
          <div>
            <span className="break-all">{collection.title} </span>
            <span className="text-sm font-normal text-base-content/70">
              {`(${collection.geometry_type})`}
            </span>
          </div>
        </div>
        <div>
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
                    requestDelete({
                      id: collectionId,
                      title: collection.title,
                    });
                  }}
                >
                  <Trash2 size={12} />
                </button>
              </div>
            )}
            <div className="flex gap-1">
              <button
                type="button"
                className="btn btn-outline btn-xs btn-square px-1 !shadow-btn"
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
          </div>
        </div>
      </summary>
      <div className="collapse-content text-sm pb-1 mt-1">
        {collection.description && (
          <div className="flex mb-1">
            <p className="text-xs text-base-content/90">
              {collection.description}
            </p>
          </div>
        )}
        <div className="flex justify-between mb-2">
          <span className="text-base-content/60 text-xs whitespace-nowrap">
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
            archivedCount={
              data?.features.filter((f) => f.properties.status === "ARCHIVED")
                .length
            }
          />
        </div>
        {data && (
          <ProjectCollection
            data={data}
            showArchived={showArchived}
            visibilityMap={visibilityMap}
            setVisibilityMap={setVisibilityMap}
          />
        )}
      </div>
    </details>
  );
};
