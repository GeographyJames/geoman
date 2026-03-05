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
  const serviceType = selectedService?.service_type ?? "";

  const arcgisServiceName = watch("arcgis_service_name");
  const arcgisLayerId = watch("arcgis_layer_id");
  const mvtUrl = watch("mvt_url");

  const sourceComplete =
    serviceType === "ArcGISRest" ? !!(arcgisServiceName && arcgisLayerId) :
    serviceType === "MVT" ? !!mvtUrl :
    !!selectedServiceId;

  const onSubmit = (data: LayerFormData) => {
    let source: unknown;
    if (serviceType === "MVT") {
      source = { url: data.mvt_url };
    } else if (serviceType === "ArcGISRest") {
      source = { service_name: data.arcgis_service_name, layer_id: Number(data.arcgis_layer_id) };
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
      <fieldset className="fieldset">
        <legend className="fieldset-legend">Provider</legend>
        <select
          className="select w-full"
          value={selectedProviderId}
          onChange={(e) => {
            setSelectedProviderId(e.target.value);
            reset((prev) => ({ ...prev, service_id: "" }));
          }}
        >
          <option value="" disabled>Select a provider…</option>
          {providers.map((p) => (
            <option key={p.id} value={p.id}>{p.name}</option>
          ))}
        </select>
      </fieldset>

      {selectedProviderId && (
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Service</legend>
          <select
            className={`select w-full ${errors.service_id ? "select-error" : ""}`}
            {...register("service_id", { required: "Service is required" })}
          >
            <option value="" disabled>Select a service…</option>
            {filteredServices.map((s) => (
              <option key={s.id} value={s.id}>{s.name} ({s.service_type})</option>
            ))}
          </select>
          {errors.service_id && <p className="label text-error">{errors.service_id.message}</p>}
        </fieldset>
      )}

      {selectedServiceId && (
        <LayerForm
          register={register}
          errors={errors}
          serviceType={serviceType}
          serviceBaseUrl={selectedService?.base_url}
          sourceComplete={sourceComplete}
          mode="create"
        />
      )}

      <div className="modal-action">
        <CancelButton onClick={() => { reset(); setSelectedProviderId(""); closeDialog(); }} disabled={isPending} />
        {sourceComplete && (
          <SubmitButton text="Create layer" loadingText="Creating..." loading={isPending} />
        )}
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
