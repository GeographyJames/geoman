import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchDataProviderService } from "@/hooks/api/usePatchDataProviderService";
import { ApiError } from "@/lib/api";
import type { DataProviderService, DataProviderServiceType } from "@/domain/data_provider/types";

const SERVICE_TYPES: DataProviderServiceType[] = [
  "ImageWMS", "TileWMS", "WMTS", "WFS", "ArcGISRest", "MVT", "OGCAPIFeatures", "XYZ",
];

interface FormData {
  name: string;
  service_type: DataProviderServiceType;
  base_url: string;
}

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

  const {
    register,
    handleSubmit,
    formState: { errors, isDirty, dirtyFields },
  } = useForm<FormData>({
    values: service
      ? {
          name: service.name,
          service_type: service.service_type,
          base_url: service.base_url,
        }
      : undefined,
  });

  const onSubmit = (data: FormData) => {
    if (!service) return;
    const patch: Parameters<typeof patchService>[0]["patch"] = {};
    if (dirtyFields.name) patch.name = data.name;
    if (dirtyFields.service_type) patch.service_type = data.service_type;
    if (dirtyFields.base_url) patch.base_url = data.base_url;

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

  const handleCancel = () => { closeDialog(); onClose(); };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div className="form-control">
        <label className="label" htmlFor="edit-service-name">
          <span className="label-text">Name</span>
        </label>
        <input
          id="edit-service-name"
          type="text"
          className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Name is required" })}
        />
        {errors.name && <span className="label-text-alt text-error mt-1">{errors.name.message}</span>}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="edit-service-type">
          <span className="label-text">Service type</span>
        </label>
        <select
          id="edit-service-type"
          className="select select-bordered w-full"
          {...register("service_type")}
        >
          {SERVICE_TYPES.map((t) => (
            <option key={t} value={t}>{t}</option>
          ))}
        </select>
      </div>

      <div className="form-control">
        <label className="label" htmlFor="edit-service-url">
          <span className="label-text">Base URL</span>
        </label>
        <input
          id="edit-service-url"
          type="text"
          className={`input input-bordered w-full ${errors.base_url ? "input-error" : ""}`}
          {...register("base_url", { required: "Base URL is required" })}
        />
        {errors.base_url && <span className="label-text-alt text-error mt-1">{errors.base_url.message}</span>}
      </div>

      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
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
