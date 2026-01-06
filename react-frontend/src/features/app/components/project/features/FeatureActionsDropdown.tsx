import { ActionsDropdown } from "@/components/ActionsDropdown";
import { ToggleArchivedStatus } from "@/components/ToggleArchivedStatus";
import type { ProjectCollectionItem } from "@/domain/projectCollectionItems/outputDTO";

export const FeatureActionsDropdown = ({
  item,
}: {
  item: ProjectCollectionItem;
}) => {
  return (
    <ActionsDropdown
      id={`c${item.properties.collection_id}-item${item.id}`}
      style="bg-base-100"
    >
      <li>
        <button>download shapefile</button>
        <button>duplicate</button>
        <button>edit</button>
        <ToggleArchivedStatus
          archived={item.properties.status === "ARCHIVED"}
          onClick={(e) => {
            const popover = (e.currentTarget as HTMLElement).closest(
              "[popover]"
            ) as HTMLElement | null;
            popover?.hidePopover();
          }}
        />
      </li>
      <li>
        <button>delete</button>
      </li>
    </ActionsDropdown>
  );
};
