import { ActionsDropdown } from "@/components/ActionsDropdown";
import { ToggleArchivedStatus } from "@/components/ToggleArchivedStatus";
import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";
import { usePatchProjectFeature } from "@/hooks/api/projectFeature.ts/usePatchProjectFeature";
import { useFlash } from "@/features/app/contexts/FlashMessageContext";
import { useEditFeature } from "@/features/app/contexts/EditFeatureContext";
import { DeleteFeatureButton } from "./DeleteFeatureButton";

export const FeatureActionsDropdown = ({
  item,
}: {
  item: ProjectCollectionItem;
}) => {
  const { mutate: patchProjectFeature } = usePatchProjectFeature();
  const { addFlash } = useFlash();
  const { requestEdit } = useEditFeature();
  const action =
    item.properties.status === "ARCHIVED" ? "unarchive" : "archive";
  return (
    <ActionsDropdown
      id={`c${item.properties.collection_id}-item${item.id}`}
      style="bg-base-100"
    >
      <li>
        <button>download shapefile</button>
        <button onClick={() => requestEdit(item)}>edit</button>
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
      </li>
      <li>
        <DeleteFeatureButton feature={item} />
      </li>
    </ActionsDropdown>
  );
};
