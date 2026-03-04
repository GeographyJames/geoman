import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useDeleteDataProvider } from "@/hooks/api/useDeleteDataProvider";
import { ApiError } from "@/lib/api";
import type { DataProvider } from "@/domain/data_provider/types";

const MODAL_ID = "delete_data_provider";

const DeleteProviderInner = ({
  provider,
  onClose,
}: {
  provider: DataProvider | null;
  onClose: () => void;
}) => {
  const { mutate: deleteProvider, isPending } = useDeleteDataProvider();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!provider) return;
    deleteProvider(provider.id, {
      onSuccess: () => { closeDialog(); onClose(); },
      onError: (error) => {
        const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
        addError(`Unable to delete provider: ${message}`);
      },
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {provider && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{provider.name}</span>? This will also delete all its services and layers.
        </p>
      )}
      <p><span className="font-bold">This action cannot be undone.</span></p>
      <div className="modal-action">
        <CancelButton onClick={() => { closeDialog(); onClose(); }} disabled={isPending} />
        <SubmitButton text="Delete" colour="btn-error" loadingText="Deleting..." loading={isPending} />
      </div>
    </form>
  );
};

export const DeleteProviderForm = ({
  provider,
  onClose,
}: {
  provider: DataProvider | null;
  onClose: () => void;
}) => (
  <Modal id={MODAL_ID} title="Delete provider" onClose={onClose}>
    <DeleteProviderInner provider={provider} onClose={onClose} />
  </Modal>
);

export const openDeleteProviderModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
