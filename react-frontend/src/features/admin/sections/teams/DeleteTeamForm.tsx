import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useDeleteTeam } from "@/hooks/api/useDeleteTeam";
import { ApiError } from "@/lib/api";
import type Team from "@/domain/team/entity";

const MODAL_ID = "delete_team";

const DeleteTeamInner = ({ team, onClose }: { team: Team | null; onClose: () => void }) => {
  const { mutate: deleteTeam, isPending } = useDeleteTeam();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!team) return;
    deleteTeam(team.id, {
      onSuccess: () => {
        closeDialog();
        onClose();
      },
      onError: (error) => {
        const message =
          error instanceof ApiError && error.status === 500
            ? "internal server error"
            : error.message;
        addError(`Unable to delete team: ${message}`);
      },
    });
  };

  const handleCancel = () => {
    closeDialog();
    onClose();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {team && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{team.name}</span>?
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

export const DeleteTeamForm = ({ team, onClose }: { team: Team | null; onClose: () => void }) => (
  <Modal id={MODAL_ID} title="Delete team" onClose={onClose}>
    <DeleteTeamInner team={team} onClose={onClose} />
  </Modal>
);

export const openDeleteTeamModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
