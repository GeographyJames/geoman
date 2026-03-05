import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostDataProviderService } from "@/hooks/api/usePostDataProviderService";
import { ApiError } from "@/lib/api";
import type { DataProvider } from "@/domain/data_provider/types";
import {
  ServiceForm,
  SERVICE_FORM_DEFAULTS,
  type ServiceFormData,
} from "./ServiceForm";

const MODAL_ID = "create_data_provider_service";

const CreateServiceInner = ({
  provider,
}: {
  provider: DataProvider | null;
}) => {
  const { mutate: postService, isPending } = usePostDataProviderService();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    reset,
    watch,
    setValue,
    formState: { errors },
  } = useForm<ServiceFormData>({
    defaultValues: SERVICE_FORM_DEFAULTS,
  });

  const serviceType = watch("service_type");
  const baseUrl = watch("base_url");
  const needsBaseUrl = serviceType !== "" && serviceType !== "MVT";

  useEffect(() => {
    if (provider && serviceType) {
      setValue("name", `${provider.name} ${serviceType}`);
    }
  }, [provider, serviceType, setValue]);

  const onSubmit = (data: ServiceFormData) => {
    if (!provider) return;
    postService(
      {
        provider_id: provider.id,
        name: data.name,
        service_type: data.service_type,
        base_url: needsBaseUrl ? data.base_url : null,
      },
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
          addError(`Unable to create service: ${message}`);
        },
      },
    );
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      {provider && (
        <p className="text-sm text-base-content/60">
          Adding service to{" "}
          <span className="font-medium text-base-content">{provider.name}</span>
        </p>
      )}
      <ServiceForm
        register={register}
        errors={errors}
        needsBaseUrl={needsBaseUrl}
        mode="create"
        serviceType={serviceType}
        baseUrl={baseUrl}
      />
      <div className="modal-action">
        <CancelButton
          onClick={() => {
            reset();
            closeDialog();
          }}
          disabled={isPending}
        />
        <SubmitButton
          text="Create service"
          loadingText="Creating..."
          loading={isPending}
        />
      </div>
    </form>
  );
};

export const CreateServiceForm = ({
  provider,
}: {
  provider: DataProvider | null;
}) => (
  <Modal id={MODAL_ID} title="Add service">
    <CreateServiceInner provider={provider} />
  </Modal>
);

export const openCreateServiceModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
