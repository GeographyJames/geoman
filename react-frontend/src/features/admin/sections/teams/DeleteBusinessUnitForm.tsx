import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { useDeleteBusinessUnit } from "@/hooks/api/useDeleteBusinessUnit";
import { ApiError } from "@/lib/api";
import type { BusinessUnitOutputDto } from "@/domain/business_unit/outputDto";

const MODAL_ID = "delete_business_unit";

const DeleteBusinessUnitInner = ({
  businessUnit,
  onClose,
}: {
  businessUnit: BusinessUnitOutputDto | null;
  onClose: () => void;
}) => {
  const { mutate: deleteBusinessUnit, isPending } = useDeleteBusinessUnit();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!businessUnit) return;
    deleteBusinessUnit(businessUnit.id, {
      onSuccess: () => {
        closeDialog();
        onClose();
      },
      onError: (error) => {
        const message =
          error instanceof ApiError && error.status === 500
            ? "internal server error"
            : error.message;
        addError(`Unable to delete business unit: ${message}`);
      },
    });
  };

  const handleCancel = () => {
    closeDialog();
    onClose();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {businessUnit && (
        <p>
          Are you sure you want to permanently delete{" "}
          <span className="font-bold">{businessUnit.name}</span>?
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

export const DeleteBusinessUnitForm = ({
  businessUnit,
  onClose,
}: {
  businessUnit: BusinessUnitOutputDto | null;
  onClose: () => void;
}) => (
  <Modal id={MODAL_ID} title="Delete business unit" onClose={onClose}>
    <DeleteBusinessUnitInner businessUnit={businessUnit} onClose={onClose} />
  </Modal>
);

export const openDeleteBusinessUnitModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
