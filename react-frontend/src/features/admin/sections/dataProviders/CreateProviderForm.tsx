import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostDataProvider } from "@/hooks/api/usePostDataProvider";
import { ApiError } from "@/lib/api";

interface FormData {
  name: string;
  description: string;
  country_code: string;
  subdivision: string;
}

const MODAL_ID = "create_data_provider";

const CreateProviderInner = () => {
  const { mutate: postProvider, isPending } = usePostDataProvider();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<FormData>({ defaultValues: { name: "", description: "", country_code: "", subdivision: "" } });

  const onSubmit = (data: FormData) => {
    postProvider(
      {
        name: data.name,
        description: data.description || null,
        country_code: data.country_code || null,
        subdivision: data.subdivision || null,
      },
      {
        onSuccess: () => { reset(); closeDialog(); },
        onError: (error) => {
          const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
          addError(`Unable to create provider: ${message}`);
        },
      },
    );
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div className="form-control">
        <label className="label" htmlFor="provider-name">
          <span className="label-text">Name</span>
        </label>
        <input
          id="provider-name"
          type="text"
          placeholder="e.g. Natural England"
          className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Name is required" })}
        />
        {errors.name && <span className="label-text-alt text-error mt-1">{errors.name.message}</span>}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="provider-description">
          <span className="label-text">Description</span>
          <span className="label-text-alt text-base-content/50">optional</span>
        </label>
        <input
          id="provider-description"
          type="text"
          className="input input-bordered w-full"
          {...register("description")}
        />
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="provider-country">
            <span className="label-text">Country code</span>
            <span className="label-text-alt text-base-content/50">optional</span>
          </label>
          <input
            id="provider-country"
            type="text"
            placeholder="e.g. GB"
            className="input input-bordered w-full"
            {...register("country_code")}
          />
        </div>
        <div className="form-control">
          <label className="label" htmlFor="provider-subdivision">
            <span className="label-text">Subdivision</span>
            <span className="label-text-alt text-base-content/50">optional</span>
          </label>
          <input
            id="provider-subdivision"
            type="text"
            placeholder="e.g. GB-ENG"
            className="input input-bordered w-full"
            {...register("subdivision")}
          />
        </div>
      </div>

      <div className="modal-action">
        <CancelButton onClick={() => { reset(); closeDialog(); }} disabled={isPending} />
        <SubmitButton text="Create provider" loadingText="Creating..." loading={isPending} />
      </div>
    </form>
  );
};

export const CreateProviderForm = () => (
  <Modal id={MODAL_ID} title="Add provider">
    <CreateProviderInner />
  </Modal>
);

export const openCreateProviderModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
