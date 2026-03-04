import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useDeleteDataProviderService } from "@/hooks/api/useDeleteDataProviderService";
import { ApiError } from "@/lib/api";
import type { DataProviderService } from "@/domain/data_provider/types";

const MODAL_ID = "delete_data_provider_service";

const DeleteServiceInner = ({
  service,
  onClose,
}: {
  service: DataProviderService | null;
  onClose: () => void;
}) => {
  const { mutate: deleteService, isPending } = useDeleteDataProviderService();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!service) return;
    deleteService(service.id, {
      onSuccess: () => { closeDialog(); onClose(); },
      onError: (error) => {
        const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
        addError(`Unable to delete service: ${message}`);
      },
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {service && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{service.name}</span>? This will also delete all its layers.
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

export const DeleteServiceForm = ({
  service,
  onClose,
}: {
  service: DataProviderService | null;
  onClose: () => void;
}) => (
  <Modal id={MODAL_ID} title="Delete service" onClose={onClose}>
    <DeleteServiceInner service={service} onClose={onClose} />
  </Modal>
);

export const openDeleteServiceModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
