import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostDataProvider } from "@/hooks/api/usePostDataProvider";
import { ApiError } from "@/lib/api";
import { ProviderForm, PROVIDER_FORM_DEFAULTS, type ProviderFormData } from "./ProviderForm";

const MODAL_ID = "create_data_provider";

const CreateProviderInner = () => {
  const { mutate: postProvider, isPending } = usePostDataProvider();
  const { addError, closeDialog } = useModal();

  const { register, handleSubmit, reset, formState: { errors } } = useForm<ProviderFormData>({
    defaultValues: PROVIDER_FORM_DEFAULTS,
  });

  const onSubmit = (data: ProviderFormData) => {
    postProvider(
      {
        name: data.name,
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
      <ProviderForm register={register} errors={errors} mode="create" />
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
