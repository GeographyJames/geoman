import { useState } from "react";
import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostDataProviderLayer } from "@/hooks/api/usePostDataProviderLayer";
import { useDataProviderServices } from "@/hooks/api/useDataProviderServices";
import { useDataProviders } from "@/hooks/api/useDataProviders";
import { ApiError } from "@/lib/api";
import type { LayerCategory } from "@/domain/data_provider/types";

interface FormData {
  service_id: string;
  name: string;
  abbreviation: string;
  source: string;
  category: LayerCategory;
  description: string;
  style_config: string;
  display_options: string;
  country_code: string;
  subdivision: string;
  sort_order: string;
}

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
    formState: { errors },
    setError,
  } = useForm<FormData>({
    defaultValues: {
      service_id: "",
      name: "",
      abbreviation: "",
      source: "{}",
      category: "overlay",
      description: "",
      style_config: "",
      display_options: "",
      country_code: "",
      subdivision: "",
      sort_order: "",
    },
  });

  const filteredServices = selectedProviderId
    ? services.filter((s) => s.provider_id === Number(selectedProviderId))
    : [];

  const onSubmit = (data: FormData) => {
    let source: unknown;
    try {
      source = JSON.parse(data.source);
    } catch {
      setError("source", { message: "Must be valid JSON" });
      return;
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
            // reset service selection when provider changes
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

      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="layer-name">
            <span className="label-text">Name</span>
          </label>
          <input
            id="layer-name"
            type="text"
            placeholder="e.g. SSSI England"
            className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
            {...register("name", { required: "Name is required" })}
          />
          {errors.name && <span className="label-text-alt text-error mt-1">{errors.name.message}</span>}
        </div>

        <div className="form-control">
          <label className="label" htmlFor="layer-abbr">
            <span className="label-text">Abbreviation</span>
            <span className="label-text-alt text-base-content/50">optional</span>
          </label>
          <input
            id="layer-abbr"
            type="text"
            placeholder="e.g. SSSI"
            className="input input-bordered w-full"
            {...register("abbreviation")}
          />
        </div>
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="layer-category">
            <span className="label-text">Category</span>
          </label>
          <select
            id="layer-category"
            className="select select-bordered w-full"
            {...register("category")}
          >
            <option value="overlay">Overlay</option>
            <option value="basemap">Base map</option>
          </select>
        </div>

        <div className="form-control">
          <label className="label" htmlFor="layer-sort-order">
            <span className="label-text">Sort order</span>
            <span className="label-text-alt text-base-content/50">optional</span>
          </label>
          <input
            id="layer-sort-order"
            type="number"
            className="input input-bordered w-full"
            {...register("sort_order")}
          />
        </div>
      </div>

      <div className="form-control">
        <label className="label" htmlFor="layer-description">
          <span className="label-text">Description</span>
          <span className="label-text-alt text-base-content/50">optional</span>
        </label>
        <input
          id="layer-description"
          type="text"
          className="input input-bordered w-full"
          {...register("description")}
        />
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="layer-country">
            <span className="label-text">Country code</span>
            <span className="label-text-alt text-base-content/50">optional</span>
          </label>
          <input
            id="layer-country"
            type="text"
            placeholder="e.g. GB"
            className="input input-bordered w-full"
            {...register("country_code")}
          />
        </div>
        <div className="form-control">
          <label className="label" htmlFor="layer-subdivision">
            <span className="label-text">Subdivision</span>
            <span className="label-text-alt text-base-content/50">optional</span>
          </label>
          <input
            id="layer-subdivision"
            type="text"
            placeholder="e.g. GB-ENG"
            className="input input-bordered w-full"
            {...register("subdivision")}
          />
        </div>
      </div>

      <div className="form-control">
        <label className="label" htmlFor="layer-source">
          <span className="label-text">Source (JSON)</span>
        </label>
        <textarea
          id="layer-source"
          rows={3}
          className={`textarea textarea-bordered w-full font-mono text-xs ${errors.source ? "textarea-error" : ""}`}
          {...register("source", { required: "Source is required" })}
        />
        {errors.source && <span className="label-text-alt text-error mt-1">{errors.source.message}</span>}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="layer-style">
          <span className="label-text">Style config (JSON)</span>
          <span className="label-text-alt text-base-content/50">optional</span>
        </label>
        <textarea
          id="layer-style"
          rows={2}
          className={`textarea textarea-bordered w-full font-mono text-xs ${errors.style_config ? "textarea-error" : ""}`}
          {...register("style_config")}
        />
        {errors.style_config && <span className="label-text-alt text-error mt-1">{errors.style_config.message}</span>}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="layer-display-options">
          <span className="label-text">Display options (JSON)</span>
          <span className="label-text-alt text-base-content/50">optional</span>
        </label>
        <textarea
          id="layer-display-options"
          rows={2}
          className={`textarea textarea-bordered w-full font-mono text-xs ${errors.display_options ? "textarea-error" : ""}`}
          {...register("display_options")}
        />
        {errors.display_options && <span className="label-text-alt text-error mt-1">{errors.display_options.message}</span>}
      </div>

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
