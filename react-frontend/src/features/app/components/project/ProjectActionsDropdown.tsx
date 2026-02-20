import { ActionsDropdown } from "@/components/ActionsDropdown";
import { ToggleArchivedStatus } from "@/components/ToggleArchivedStatus";
import type Project from "@/domain/project/entity";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { useFlash } from "@/features/app/contexts/FlashMessageContext";
import { useEditProject } from "../../contexts/EditProjectContext";
import { useDeleteProject } from "../../contexts/DeleteProjectContext";
import { useAddFeature } from "../../contexts/AddFeatureContext";
export const ProjectActionsDropdown = ({
  item,
  id,
  zoomToProject,
  hasExtent,
}: {
  item: Project;
  id: string;
  zoomToProject?: () => void;
  hasExtent?: boolean;
}) => {
  const { mutate: patchProject } = usePatchProject();
  const { addFlash } = useFlash();
  const action = item.archived ? "unarchive" : "archive";
  const { requestEdit } = useEditProject();
  const { requestDelete } = useDeleteProject();
  const { requestAddFeature } = useAddFeature();
  return (
    <ActionsDropdown id={id}>
      {hasExtent && (
        <li>
          <button onClick={(e) => {
            zoomToProject?.();
            const popover = (e.currentTarget as HTMLElement).closest(
              "[popover]",
            ) as HTMLElement | null;
            popover?.hidePopover();
          }}>zoom to project</button>
        </li>
      )}
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
        <button onClick={() => requestAddFeature(item)}>add feature</button>
      </li>
      <li>
        <button>create figure</button>
      </li>
      <li>
        <button>manage project members</button>
      </li>
      <li>
        <button onClick={() => requestDelete(item)}>delete</button>
      </li>
    </ActionsDropdown>
  );
};
