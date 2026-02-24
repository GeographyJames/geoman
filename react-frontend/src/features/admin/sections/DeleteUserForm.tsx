import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useDeleteUser } from "@/hooks/api/useDeleteUser";
import { ApiError } from "@/lib/api";
import type User from "@/domain/user/entity";

const MODAL_ID = "delete_user";

const DeleteUserInner = ({ user, onClose }: { user: User | null; onClose: () => void }) => {
  const { mutate: deleteUser, isPending } = useDeleteUser();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!user) return;
    deleteUser(user.id, {
      onSuccess: () => {
        closeDialog();
        onClose();
      },
      onError: (error) => {
        const message =
          error instanceof ApiError && error.status === 500
            ? "internal server error"
            : error.message;
        addError(`Unable to delete user: ${message}`);
      },
    });
  };

  const handleCancel = () => {
    closeDialog();
    onClose();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {user && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">
            {user.firstName} {user.lastName}
          </span>
          ?
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

export const DeleteUserForm = ({ user, onClose }: { user: User | null; onClose: () => void }) => (
  <Modal id={MODAL_ID} title="Delete user" onClose={onClose}>
    <DeleteUserInner user={user} onClose={onClose} />
  </Modal>
);

export const openDeleteUserModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
