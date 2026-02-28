import { ActionsDropdown } from "@/components/ActionsDropdown";
import { ToggleArchivedStatus } from "@/components/ToggleArchivedStatus";
import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";
import { usePatchProjectFeature } from "@/hooks/api/projectFeature.ts/usePatchProjectFeature";
import { useDownloadFeatureShapefile } from "@/hooks/api/projectFeature.ts/useDownloadFeatureShapefile";
import { useFlash } from "@/features/app/contexts/FlashMessageContext";
import { useEditFeature } from "@/features/app/contexts/EditFeatureContext";
import { useDuplicateTurbineLayoutContext } from "@/features/app/contexts/DuplicateTurbineLayoutContext";
import { DeleteFeatureButton } from "./DeleteFeatureButton";
import { TURBINE_LAYOUT_CCOLLECTION_ID } from "@/constants";

export const FeatureActionsDropdown = ({
  item,
  zoomToFeature,
  projectSlug,
  collectionSlug,
}: {
  item: ProjectCollectionItem;
  zoomToFeature: () => void;
  projectSlug: string;
  collectionSlug: string;
}) => {
  const { mutate: patchProjectFeature } = usePatchProjectFeature();
  const { download, isLoading: isDownloading } = useDownloadFeatureShapefile();
  const { addFlash } = useFlash();
  const { requestEdit } = useEditFeature();
  const { requestDuplicate } = useDuplicateTurbineLayoutContext();
  const action =
    item.properties.status === "ARCHIVED" ? "unarchive" : "archive";
  return (
    <ActionsDropdown
      id={`c${item.properties.collection_id}-item${item.id}`}
      style="bg-base-100"
    >
      <li>
        <button onClick={(e) => {
          zoomToFeature();
          const popover = (e.currentTarget as HTMLElement).closest(
            "[popover]",
          ) as HTMLElement | null;
          popover?.hidePopover();
        }}>zoom to feature</button>
      </li>
      <li>
        <a
          href={`${__URLS__.api.base}${__URLS__.api.project_features}/${projectSlug}/${collectionSlug}/${item.id}?format=shapefile`}
          aria-disabled={isDownloading}
          onClick={async (e) => {
            e.preventDefault();
            if (isDownloading) return;
            const popover = (e.currentTarget as HTMLElement).closest(
              "[popover]",
            ) as HTMLElement | null;
            await download(item.id, projectSlug, collectionSlug);
            popover?.hidePopover();
          }}
        >
          {isDownloading ? "downloading..." : "download shapefile"}
        </a>
      </li>
      <li>
        <button onClick={() => requestEdit(item)}>edit</button>
      </li>
      {item.properties.collection_id === TURBINE_LAYOUT_CCOLLECTION_ID && (
        <li>
          <button
            onClick={(e) => {
              requestDuplicate(item);
              const popover = (e.currentTarget as HTMLElement).closest(
                "[popover]",
              ) as HTMLElement | null;
              popover?.hidePopover();
            }}
          >
            duplicate
          </button>
        </li>
      )}
      <li>
        {item.properties.is_primary ? (
          <div
            className="tooltip tooltip-left flex"
            data-tip={`Unable to archive primary feature`}
          >
            <button className="text-base-content/50  ">archive</button>
          </div>
        ) : (
          <ToggleArchivedStatus
            archived={item.properties.status === "ARCHIVED"}
            disabled={item.properties.is_primary}
            onClick={(e) => {
              patchProjectFeature(
                {
                  projectId: item.properties.project_id,
                  collectionId: item.properties.collection_id.toString(),
                  id: item.id,
                  dto: {
                    status:
                      item.properties.status === "ARCHIVED"
                        ? "ACTIVE"
                        : "ARCHIVED",
                  },
                },
                {
                  onError: (error) => {
                    addFlash(
                      `Unable to ${action} feature: ${error.message}`,
                      "error",
                    );
                  },
                },
              );
              const popover = (e.currentTarget as HTMLElement).closest(
                "[popover]",
              ) as HTMLElement | null;
              popover?.hidePopover();
            }}
          />
        )}
      </li>

      <li>
        {item.properties.is_primary ? (
          <div
            className="tooltip tooltip-left flex"
            data-tip={`Unable to delete primary feature`}
          >
            <button className="text-base-content/50  ">delete</button>
          </div>
        ) : (
          <DeleteFeatureButton feature={item} />
        )}
      </li>
    </ActionsDropdown>
  );
};
