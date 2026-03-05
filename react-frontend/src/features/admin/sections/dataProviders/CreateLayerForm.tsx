import { useState } from "react";
import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostDataProviderLayer } from "@/hooks/api/usePostDataProviderLayer";
import { useDataProviderServices } from "@/hooks/api/useDataProviderServices";
import { useDataProviders } from "@/hooks/api/useDataProviders";
import { ApiError } from "@/lib/api";
import { LayerForm, LAYER_FORM_DEFAULTS, type LayerFormData } from "./LayerForm";

const MODAL_ID = "create_data_provider_layer";

const CreateLayerInner = () => {
  const { mutate: postLayer, isPending } = usePostDataProviderLayer();
  const { data: services = [] } = useDataProviderServices();
  const { data: providers = [] } = useDataProviders();
  const { addError, closeDialog } = useModal();
  const [selectedProviderId, setSelectedProviderId] = useState<string>("");

  const {
    register,
    handleSubmit,
    reset,
    watch,
    formState: { errors },
    setError,
  } = useForm<LayerFormData>({ defaultValues: LAYER_FORM_DEFAULTS });

  const filteredServices = selectedProviderId
    ? services.filter((s) => s.provider_id === Number(selectedProviderId))
    : [];

  const selectedServiceId = watch("service_id");
  const selectedService = filteredServices.find((s) => String(s.id) === selectedServiceId);
  const isMVT = selectedService?.service_type === "MVT";

  const onSubmit = (data: LayerFormData) => {
    let source: unknown;
    if (isMVT) {
      source = { url: data.mvt_url };
    } else {
      try {
        source = JSON.parse(data.source);
      } catch {
        setError("source", { message: "Must be valid JSON" });
        return;
      }
    }

    let style_config: unknown = undefined;
    if (data.style_config.trim()) {
      try {
        style_config = JSON.parse(data.style_config);
      } catch {
        setError("style_config", { message: "Must be valid JSON" });
        return;
      }
    }

    let display_options: unknown = undefined;
    if (data.display_options.trim()) {
      try {
        display_options = JSON.parse(data.display_options);
      } catch {
        setError("display_options", { message: "Must be valid JSON" });
        return;
      }
    }

    postLayer(
      {
        service_id: Number(data.service_id),
        name: data.name,
        abbreviation: data.abbreviation || null,
        source,
        category: data.category,
        description: data.description || null,
        style_config,
        display_options,
        country_code: data.country_code || null,
        subdivision: data.subdivision || null,
        sort_order: data.sort_order ? Number(data.sort_order) : undefined,
      },
      {
        onSuccess: () => { reset(); setSelectedProviderId(""); closeDialog(); },
        onError: (error) => {
          const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
          addError(`Unable to create layer: ${message}`);
        },
      },
    );
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div className="form-control">
        <label className="label" htmlFor="layer-provider">
          <span className="label-text">Provider</span>
        </label>
        <select
          id="layer-provider"
          className="select select-bordered w-full"
          value={selectedProviderId}
          onChange={(e) => {
            setSelectedProviderId(e.target.value);
            reset((prev) => ({ ...prev, service_id: "" }));
          }}
        >
          <option value="">Select a provider…</option>
          {providers.map((p) => (
            <option key={p.id} value={p.id}>{p.name}</option>
          ))}
        </select>
      </div>

      <div className="form-control">
        <label className="label" htmlFor="layer-service">
          <span className="label-text">Service</span>
        </label>
        <select
          id="layer-service"
          className={`select select-bordered w-full ${errors.service_id ? "select-error" : ""}`}
          disabled={!selectedProviderId}
          {...register("service_id", { required: "Service is required" })}
        >
          <option value="">
            {selectedProviderId ? "Select a service…" : "Select a provider first"}
          </option>
          {filteredServices.map((s) => (
            <option key={s.id} value={s.id}>{s.name} ({s.service_type})</option>
          ))}
        </select>
        {errors.service_id && <span className="label-text-alt text-error mt-1">{errors.service_id.message}</span>}
      </div>

      <LayerForm register={register} errors={errors} isMVT={isMVT} mode="create" />

      <div className="modal-action">
        <CancelButton onClick={() => { reset(); setSelectedProviderId(""); closeDialog(); }} disabled={isPending} />
        <SubmitButton text="Create layer" loadingText="Creating..." loading={isPending} />
      </div>
    </form>
  );
};

export const CreateLayerForm = () => (
  <Modal id={MODAL_ID} title="Add layer">
    <CreateLayerInner />
  </Modal>
);

export const openCreateLayerModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
