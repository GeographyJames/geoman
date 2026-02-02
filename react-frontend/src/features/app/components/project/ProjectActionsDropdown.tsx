import { ActionsDropdown } from "@/components/ActionsDropdown";
import { ToggleArchivedStatus } from "@/components/ToggleArchivedStatus";
import type Project from "@/domain/project/entity";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { useFlash } from "@/features/app/contexts/FlashMessageContext";
import { Link } from "@tanstack/react-router";

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
                  addFlash(`Unable to ${action} project: ${error.message}`, "error");
                },
              },
            );

            const popover = (e.currentTarget as HTMLElement).closest(
              "[popover]"
            ) as HTMLElement | null;
            popover?.hidePopover();
          }}
        />
      </li>
      <li>
        <Link to="/project/$slug" params={{ slug: item.slug }}>
          admin
        </Link>
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
