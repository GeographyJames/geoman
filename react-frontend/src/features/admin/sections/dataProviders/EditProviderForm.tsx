import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchDataProvider } from "@/hooks/api/usePatchDataProvider";
import { ApiError } from "@/lib/api";
import type { DataProvider } from "@/domain/data_provider/types";

interface FormData {
  name: string;
  description: string;
  country_code: string;
  subdivision: string;
}

const MODAL_ID = "edit_data_provider";

const EditProviderInner = ({
  provider,
  onClose,
}: {
  provider: DataProvider | null;
  onClose: () => void;
}) => {
  const { mutate: patchProvider, isPending } = usePatchDataProvider();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    formState: { errors, isDirty, dirtyFields },
  } = useForm<FormData>({
    values: provider
      ? {
          name: provider.name,
          description: provider.description ?? "",
          country_code: provider.country_code ?? "",
          subdivision: provider.subdivision ?? "",
        }
      : undefined,
  });

  const onSubmit = (data: FormData) => {
    if (!provider) return;
    const patch: Parameters<typeof patchProvider>[0]["patch"] = {};
    if (dirtyFields.name) patch.name = data.name;
    if (dirtyFields.description) patch.description = data.description || null;
    if (dirtyFields.country_code) patch.country_code = data.country_code || null;
    if (dirtyFields.subdivision) patch.subdivision = data.subdivision || null;

    patchProvider(
      { id: provider.id, patch },
      {
        onSuccess: () => { closeDialog(); onClose(); },
        onError: (error) => {
          const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
          addError(`Unable to update provider: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => { closeDialog(); onClose(); };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div className="form-control">
        <label className="label" htmlFor="edit-provider-name">
          <span className="label-text">Name</span>
        </label>
        <input
          id="edit-provider-name"
          type="text"
          className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Name is required" })}
        />
        {errors.name && <span className="label-text-alt text-error mt-1">{errors.name.message}</span>}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="edit-provider-description">
          <span className="label-text">Description</span>
          <span className="label-text-alt text-base-content/50">clear to remove</span>
        </label>
        <input
          id="edit-provider-description"
          type="text"
          className="input input-bordered w-full"
          {...register("description")}
        />
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="edit-provider-country">
            <span className="label-text">Country code</span>
          </label>
          <input
            id="edit-provider-country"
            type="text"
            placeholder="e.g. GB"
            className="input input-bordered w-full"
            {...register("country_code")}
          />
        </div>
        <div className="form-control">
          <label className="label" htmlFor="edit-provider-subdivision">
            <span className="label-text">Subdivision</span>
          </label>
          <input
            id="edit-provider-subdivision"
            type="text"
            placeholder="e.g. GB-ENG"
            className="input input-bordered w-full"
            {...register("subdivision")}
          />
        </div>
      </div>

      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton text="Save changes" loadingText="Saving..." loading={isPending} disabled={!isDirty} />
      </div>
    </form>
  );
};

export const EditProviderForm = ({
  provider,
  onClose,
}: {
  provider: DataProvider | null;
  onClose: () => void;
}) => (
  <Modal id={MODAL_ID} title="Edit provider" onClose={onClose}>
    <EditProviderInner provider={provider} onClose={onClose} />
  </Modal>
);

export const openEditProviderModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
