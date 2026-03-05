import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchDataProviderService } from "@/hooks/api/usePatchDataProviderService";
import { ApiError } from "@/lib/api";
import type { DataProviderService } from "@/domain/data_provider/types";
import { ServiceForm, SERVICE_FORM_DEFAULTS, type ServiceFormData } from "./ServiceForm";

const MODAL_ID = "edit_data_provider_service";

const EditServiceInner = ({
  service,
  onClose,
}: {
  service: DataProviderService | null;
  onClose: () => void;
}) => {
  const { mutate: patchService, isPending } = usePatchDataProviderService();
  const { addError, closeDialog } = useModal();

  const { register, handleSubmit, reset, watch, formState: { errors, isDirty, dirtyFields } } = useForm<ServiceFormData>({
    values: service
      ? {
          name: service.name,
          service_type: service.service_type,
          base_url: service.base_url ?? "",
        }
      : SERVICE_FORM_DEFAULTS,
  });

  const needsBaseUrl = watch("service_type") !== "MVT";

  const onSubmit = (data: ServiceFormData) => {
    if (!service) return;
    const patch: Parameters<typeof patchService>[0]["patch"] = {};
    if (dirtyFields.name) patch.name = data.name;
    if (dirtyFields.service_type) patch.service_type = data.service_type;
    if (dirtyFields.base_url) patch.base_url = needsBaseUrl ? data.base_url : null;

    patchService(
      { id: service.id, patch },
      {
        onSuccess: () => { closeDialog(); onClose(); },
        onError: (error) => {
          const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
          addError(`Unable to update service: ${message}`);
        },
      },
    );
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <ServiceForm register={register} errors={errors} needsBaseUrl={needsBaseUrl} mode="edit" />
      <div className="modal-action">
        <CancelButton onClick={() => { reset(); closeDialog(); onClose(); }} disabled={isPending} />
        <SubmitButton text="Save changes" loadingText="Saving..." loading={isPending} disabled={!isDirty} />
      </div>
    </form>
  );
};

export const EditServiceForm = ({
  service,
  onClose,
}: {
  service: DataProviderService | null;
  onClose: () => void;
}) => (
  <Modal id={MODAL_ID} title="Edit service" onClose={onClose}>
    <EditServiceInner service={service} onClose={onClose} />
  </Modal>
);

export const openEditServiceModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
