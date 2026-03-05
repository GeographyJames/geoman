import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchDataProvider } from "@/hooks/api/usePatchDataProvider";
import { ApiError } from "@/lib/api";
import type { DataProvider } from "@/domain/data_provider/types";
import { ProviderForm, PROVIDER_FORM_DEFAULTS, type ProviderFormData } from "./ProviderForm";

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

  const { register, handleSubmit, reset, formState: { errors, isDirty, dirtyFields } } = useForm<ProviderFormData>({
    values: provider
      ? {
          name: provider.name,
          country_code: provider.country_code ?? "",
          subdivision: provider.subdivision ?? "",
        }
      : PROVIDER_FORM_DEFAULTS,
  });

  const onSubmit = (data: ProviderFormData) => {
    if (!provider) return;
    const patch: Parameters<typeof patchProvider>[0]["patch"] = {};
    if (dirtyFields.name) patch.name = data.name;
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

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <ProviderForm register={register} errors={errors} mode="edit" />
      <div className="modal-action">
        <CancelButton onClick={() => { reset(); closeDialog(); onClose(); }} disabled={isPending} />
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
