import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useDeleteProject } from "@/features/app/contexts/DeleteProjectContext";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { ApiError } from "@/lib/api";

const DeleteProjectInner = () => {
  const { project, clear } = useDeleteProject();
  const { mutate: patchProject, isPending } = usePatchProject();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!project) return;
    patchProject(
      {
        id: project.id,
        dto: { status: "DELETED" },
      },
      {
        onSuccess: () => {
          closeDialog();
          clear();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to delete project: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    closeDialog();
    clear();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {project && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{project.name}</span>?
        </p>
      )}
      <p>
        <span className="font-bold">This action cannot be undone.</span>
      </p>
      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton
          text="Delete"
          colour="btn-error"
          loadingText="Deleting..."
          loading={isPending}
        />
      </div>
    </form>
  );
};

export const DeleteProjectForm = () => {
  return (
    <Modal
      id="delete_project"
      title="Delete project"
      onClose={useDeleteProject().clear}
    >
      <DeleteProjectInner />
    </Modal>
  );
};
