import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostBusinessUnit } from "@/hooks/api/usePostBusinessUnit";
import { ApiError } from "@/lib/api";

interface CreateBusinessUnitFormData {
  name: string;
}

const MODAL_ID = "create_business_unit";

const CreateBusinessUnitInner = () => {
  const { mutate: postBusinessUnit, isPending } = usePostBusinessUnit();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<CreateBusinessUnitFormData>({
    defaultValues: { name: "" },
  });

  const onSubmit = (data: CreateBusinessUnitFormData) => {
    postBusinessUnit(
      { name: data.name },
      {
        onSuccess: () => {
          reset();
          closeDialog();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to create business unit: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    reset();
    closeDialog();
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div className="form-control">
        <label className="label" htmlFor="bu-name">
          <span className="label-text">Business unit name</span>
        </label>
        <input
          id="bu-name"
          type="text"
          placeholder="Enter business unit name"
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
        <SubmitButton text="Create business unit" loadingText="Creating..." loading={isPending} />
      </div>
    </form>
  );
};

export const CreateBusinessUnitForm = () => (
  <Modal id={MODAL_ID} title="Create business unit">
    <CreateBusinessUnitInner />
  </Modal>
);

export const openCreateBusinessUnitModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
