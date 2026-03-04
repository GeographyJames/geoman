import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostDataProviderService } from "@/hooks/api/usePostDataProviderService";
import { ApiError } from "@/lib/api";
import type { DataProvider, DataProviderServiceType } from "@/domain/data_provider/types";

const SERVICE_TYPES: DataProviderServiceType[] = [
  "ImageWMS", "TileWMS", "WMTS", "WFS", "ArcGISRest", "MVT", "OGCAPIFeatures", "XYZ",
];

interface FormData {
  name: string;
  service_type: DataProviderServiceType;
  base_url: string;
}

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
    formState: { errors },
  } = useForm<FormData>({
    defaultValues: { name: "", service_type: "ImageWMS", base_url: "" },
  });

  const onSubmit = (data: FormData) => {
    if (!provider) return;
    postService(
      {
        provider_id: provider.id,
        name: data.name,
        service_type: data.service_type,
        base_url: data.base_url,
      },
      {
        onSuccess: () => { reset(); closeDialog(); },
        onError: (error) => {
          const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
          addError(`Unable to create service: ${message}`);
        },
      },
    );
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      {provider && (
        <p className="text-sm text-base-content/60">
          Adding service to <span className="font-medium text-base-content">{provider.name}</span>
        </p>
      )}

      <div className="form-control">
        <label className="label" htmlFor="service-name">
          <span className="label-text">Name</span>
        </label>
        <input
          id="service-name"
          type="text"
          placeholder="e.g. Natural England WMS"
          className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Name is required" })}
        />
        {errors.name && <span className="label-text-alt text-error mt-1">{errors.name.message}</span>}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="service-type">
          <span className="label-text">Service type</span>
        </label>
        <select
          id="service-type"
          className="select select-bordered w-full"
          {...register("service_type")}
        >
          {SERVICE_TYPES.map((t) => (
            <option key={t} value={t}>{t}</option>
          ))}
        </select>
      </div>

      <div className="form-control">
        <label className="label" htmlFor="service-url">
          <span className="label-text">Base URL</span>
        </label>
        <input
          id="service-url"
          type="text"
          placeholder="https://..."
          className={`input input-bordered w-full ${errors.base_url ? "input-error" : ""}`}
          {...register("base_url", { required: "Base URL is required" })}
        />
        {errors.base_url && <span className="label-text-alt text-error mt-1">{errors.base_url.message}</span>}
      </div>

      <div className="modal-action">
        <CancelButton onClick={() => { reset(); closeDialog(); }} disabled={isPending} />
        <SubmitButton text="Create service" loadingText="Creating..." loading={isPending} />
      </div>
    </form>
  );
};

export const CreateServiceForm = ({ provider }: { provider: DataProvider | null }) => (
  <Modal id={MODAL_ID} title="Add service">
    <CreateServiceInner provider={provider} />
  </Modal>
);

export const openCreateServiceModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
