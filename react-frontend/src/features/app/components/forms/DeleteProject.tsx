import { ModalForm } from "@/components/forms/ModalForm";
import { useDeleteProject } from "@/features/app/contexts/DeleteProjectContext";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { useFlash } from "../../contexts/FlashMessageContext";

export const DeleteProjectForm = () => {
  const { project, clear } = useDeleteProject();
  const { mutate: patchProject } = usePatchProject();
  const { addFlash } = useFlash();

  const handleSubmit = () => {
    if (project) {
      patchProject(
        {
          id: project.id,
          dto: { status: "DELETED" },
        },
        {
          onError: (error) => {
            addFlash(`Unable to delete project: ${error.message}`, "error");
          },
        },
      );
    }
  };

  return (
    <ModalForm
      id="delete_project"
      title="Delete"
      onSubmit={handleSubmit}
      onReset={handleSubmit}
      onClose={clear}
      submitButtonText="Delete"
      submitButtonColour="btn-error"
    >
      {project && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{project.name}</span>?
        </p>
      )}
      <p>
        <span className="font-bold">This action cannot be undone.</span>
      </p>
    </ModalForm>
  );
};
