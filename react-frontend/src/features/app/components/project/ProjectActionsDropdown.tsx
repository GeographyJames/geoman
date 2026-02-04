import { ActionsDropdown } from "@/components/ActionsDropdown";
import { ToggleArchivedStatus } from "@/components/ToggleArchivedStatus";
import type Project from "@/domain/project/entity";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { useFlash } from "@/features/app/contexts/FlashMessageContext";
import { useEditProject } from "../../contexts/EditProjectContext";

export const ProjectActionsDropdown = ({
  item,
  id,
}: {
  item: Project;
  id: string;
}) => {
  const { mutate: patchProject } = usePatchProject();
  const { addFlash } = useFlash();
  const action = item.archived ? "unarchive" : "archive";
  const { requestEdit } = useEditProject();
  return (
    <ActionsDropdown id={id}>
      <li>
        <ToggleArchivedStatus
          archived={item.archived}
          onClick={(e) => {
            patchProject(
              {
                id: item.id,
                dto: { status: item.archived ? "ACTIVE" : "ARCHIVED" },
              },
              {
                onError: (error) => {
                  addFlash(
                    `Unable to ${action} project: ${error.message}`,
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
        <button onClick={() => requestEdit(item)}>edit</button>
      </li>
      <li>
        <button>add data</button>
      </li>
      <li>
        <button>create figure</button>
      </li>
    </ActionsDropdown>
  );
};
