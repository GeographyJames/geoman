import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useDeleteDataProviderLayer } from "@/hooks/api/useDeleteDataProviderLayer";
import { ApiError } from "@/lib/api";
import type { DataProviderLayer } from "@/domain/data_provider/types";

const MODAL_ID = "delete_data_provider_layer";

const DeleteLayerInner = ({
  layer,
  onClose,
}: {
  layer: DataProviderLayer | null;
  onClose: () => void;
}) => {
  const { mutate: deleteLayer, isPending } = useDeleteDataProviderLayer();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!layer) return;
    deleteLayer(layer.id, {
      onSuccess: () => { closeDialog(); onClose(); },
      onError: (error) => {
        const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
        addError(`Unable to delete layer: ${message}`);
      },
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {layer && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{layer.name}</span>?
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

export const DeleteLayerForm = ({
  layer,
  onClose,
}: {
  layer: DataProviderLayer | null;
  onClose: () => void;
}) => (
  <Modal id={MODAL_ID} title="Delete layer" onClose={onClose}>
    <DeleteLayerInner layer={layer} onClose={onClose} />
  </Modal>
);

export const openDeleteLayerModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
