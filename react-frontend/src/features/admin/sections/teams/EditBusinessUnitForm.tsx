import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchBusinessUnit } from "@/hooks/api/usePatchBusinessUnit";
import { ApiError } from "@/lib/api";
import type { BusinessUnitOutputDto } from "@/domain/business_unit/outputDto";

interface EditBusinessUnitFormData {
  name: string;
}

const MODAL_ID = "edit_business_unit";

const EditBusinessUnitInner = ({
  businessUnit,
  onClose,
}: {
  businessUnit: BusinessUnitOutputDto | null;
  onClose: () => void;
}) => {
  const { mutate: patchBusinessUnit, isPending } = usePatchBusinessUnit();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    formState: { errors, isDirty },
  } = useForm<EditBusinessUnitFormData>({
    values: businessUnit ? { name: businessUnit.name } : undefined,
  });

  const onSubmit = (data: EditBusinessUnitFormData) => {
    if (!businessUnit) return;
    patchBusinessUnit(
      { id: businessUnit.id, patch: { name: data.name } },
      {
        onSuccess: () => {
          closeDialog();
          onClose();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to update business unit: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    closeDialog();
    onClose();
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div className="form-control">
        <label className="label" htmlFor="edit-bu-name">
          <span className="label-text">Business unit name</span>
        </label>
        <input
          id="edit-bu-name"
          type="text"
          className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
          autoFocus
          {...register("name", { required: "Business unit name is required" })}
        />
        {errors.name && (
          <span className="label-text-alt text-error mt-1">{errors.name.message}</span>
        )}
      </div>

      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton
          text="Save changes"
          loadingText="Saving..."
          loading={isPending}
          disabled={!isDirty}
        />
      </div>
    </form>
  );
};

export const EditBusinessUnitForm = ({
  businessUnit,
  onClose,
}: {
  businessUnit: BusinessUnitOutputDto | null;
  onClose: () => void;
}) => (
  <Modal id={MODAL_ID} title="Edit business unit" onClose={onClose}>
    <EditBusinessUnitInner businessUnit={businessUnit} onClose={onClose} />
  </Modal>
);

export const openEditBusinessUnitModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
