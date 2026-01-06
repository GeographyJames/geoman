import { ActionsDropdown } from "@/components/ActionsDropdown";
import { ToggleArchivedStatus } from "@/components/ToggleArchivedStatus";
import type Project from "@/domain/project/entity";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { Link } from "@tanstack/react-router";

export const ProjectActionsDropdown = ({ item }: { item: Project }) => {
  const { mutate: patchProject } = usePatchProject();
  return (
    <ActionsDropdown id={`p${item.id}`}>
      <li>
        <ToggleArchivedStatus
          archived={item.archived}
          onClick={(e) => {
            patchProject({
              id: item.id,
              dto: { status: item.archived ? "ACTIVE" : "ARCHIVED" },
            });

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
