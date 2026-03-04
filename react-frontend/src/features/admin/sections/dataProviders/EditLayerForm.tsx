import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchDataProviderLayer } from "@/hooks/api/usePatchDataProviderLayer";
import { ApiError } from "@/lib/api";
import type { DataProviderLayer, LayerCategory } from "@/domain/data_provider/types";

interface FormData {
  name: string;
  abbreviation: string;
  source: string;
  category: LayerCategory;
  description: string;
  enabled: boolean;
  style_config: string;
  display_options: string;
  country_code: string;
  subdivision: string;
  sort_order: string;
}

const MODAL_ID = "edit_data_provider_layer";

const EditLayerInner = ({
  layer,
  onClose,
}: {
  layer: DataProviderLayer | null;
  onClose: () => void;
}) => {
  const { mutate: patchLayer, isPending } = usePatchDataProviderLayer();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    formState: { errors, isDirty, dirtyFields },
    setError,
  } = useForm<FormData>({
    values: layer
      ? {
          name: layer.name,
          abbreviation: layer.abbreviation ?? "",
          source: JSON.stringify(layer.source, null, 2),
          category: layer.category,
          description: layer.description ?? "",
          enabled: layer.enabled,
          style_config: layer.style_config ? JSON.stringify(layer.style_config, null, 2) : "",
          display_options: layer.display_options ? JSON.stringify(layer.display_options, null, 2) : "",
          country_code: layer.country_code ?? "",
          subdivision: layer.subdivision ?? "",
          sort_order: String(layer.sort_order),
        }
      : undefined,
  });

  const onSubmit = (data: FormData) => {
    if (!layer) return;

    const patch: Parameters<typeof patchLayer>[0]["patch"] = {};

    if (dirtyFields.name) patch.name = data.name;
    if (dirtyFields.abbreviation) patch.abbreviation = data.abbreviation || null;
    if (dirtyFields.category) patch.category = data.category;
    if (dirtyFields.description) patch.description = data.description || null;
    if (dirtyFields.enabled) patch.enabled = data.enabled;
    if (dirtyFields.country_code) patch.country_code = data.country_code || null;
    if (dirtyFields.subdivision) patch.subdivision = data.subdivision || null;
    if (dirtyFields.sort_order) patch.sort_order = data.sort_order ? Number(data.sort_order) : undefined;

    if (dirtyFields.source) {
      try {
        patch.source = JSON.parse(data.source);
      } catch {
        setError("source", { message: "Must be valid JSON" });
        return;
      }
    }

    if (dirtyFields.style_config) {
      if (data.style_config.trim()) {
        try {
          patch.style_config = JSON.parse(data.style_config);
        } catch {
          setError("style_config", { message: "Must be valid JSON" });
          return;
        }
      } else {
        patch.style_config = null;
      }
    }

    if (dirtyFields.display_options) {
      if (data.display_options.trim()) {
        try {
          patch.display_options = JSON.parse(data.display_options);
        } catch {
          setError("display_options", { message: "Must be valid JSON" });
          return;
        }
      } else {
        patch.display_options = null;
      }
    }

    patchLayer(
      { id: layer.id, patch },
      {
        onSuccess: () => { closeDialog(); onClose(); },
        onError: (error) => {
          const message = error instanceof ApiError && error.status === 500 ? "internal server error" : error.message;
          addError(`Unable to update layer: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => { closeDialog(); onClose(); };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="edit-layer-name">
            <span className="label-text">Name</span>
          </label>
          <input
            id="edit-layer-name"
            type="text"
            className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
            {...register("name", { required: "Name is required" })}
          />
          {errors.name && <span className="label-text-alt text-error mt-1">{errors.name.message}</span>}
        </div>

        <div className="form-control">
          <label className="label" htmlFor="edit-layer-abbr">
            <span className="label-text">Abbreviation</span>
            <span className="label-text-alt text-base-content/50">clear to remove</span>
          </label>
          <input
            id="edit-layer-abbr"
            type="text"
            className="input input-bordered w-full"
            {...register("abbreviation")}
          />
        </div>
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="edit-layer-category">
            <span className="label-text">Category</span>
          </label>
          <select
            id="edit-layer-category"
            className="select select-bordered w-full"
            {...register("category")}
          >
            <option value="overlay">Overlay</option>
            <option value="basemap">Base map</option>
          </select>
        </div>

        <div className="form-control">
          <label className="label" htmlFor="edit-layer-sort-order">
            <span className="label-text">Sort order</span>
          </label>
          <input
            id="edit-layer-sort-order"
            type="number"
            className="input input-bordered w-full"
            {...register("sort_order")}
          />
        </div>
      </div>

      <div className="form-control">
        <label className="label" htmlFor="edit-layer-description">
          <span className="label-text">Description</span>
          <span className="label-text-alt text-base-content/50">clear to remove</span>
        </label>
        <input
          id="edit-layer-description"
          type="text"
          className="input input-bordered w-full"
          {...register("description")}
        />
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="edit-layer-country">
            <span className="label-text">Country code</span>
          </label>
          <input
            id="edit-layer-country"
            type="text"
            placeholder="e.g. GB"
            className="input input-bordered w-full"
            {...register("country_code")}
          />
        </div>
        <div className="form-control">
          <label className="label" htmlFor="edit-layer-subdivision">
            <span className="label-text">Subdivision</span>
          </label>
          <input
            id="edit-layer-subdivision"
            type="text"
            placeholder="e.g. GB-ENG"
            className="input input-bordered w-full"
            {...register("subdivision")}
          />
        </div>
      </div>

      <div className="form-control">
        <label className="label" htmlFor="edit-layer-source">
          <span className="label-text">Source (JSON)</span>
        </label>
        <textarea
          id="edit-layer-source"
          rows={3}
          className={`textarea textarea-bordered w-full font-mono text-xs ${errors.source ? "textarea-error" : ""}`}
          {...register("source", { required: "Source is required" })}
        />
        {errors.source && <span className="label-text-alt text-error mt-1">{errors.source.message}</span>}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="edit-layer-style">
          <span className="label-text">Style config (JSON)</span>
          <span className="label-text-alt text-base-content/50">clear to remove</span>
        </label>
        <textarea
          id="edit-layer-style"
          rows={2}
          className={`textarea textarea-bordered w-full font-mono text-xs ${errors.style_config ? "textarea-error" : ""}`}
          {...register("style_config")}
        />
        {errors.style_config && <span className="label-text-alt text-error mt-1">{errors.style_config.message}</span>}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="edit-layer-display-options">
          <span className="label-text">Display options (JSON)</span>
          <span className="label-text-alt text-base-content/50">clear to remove</span>
        </label>
        <textarea
          id="edit-layer-display-options"
          rows={2}
          className={`textarea textarea-bordered w-full font-mono text-xs ${errors.display_options ? "textarea-error" : ""}`}
          {...register("display_options")}
        />
        {errors.display_options && <span className="label-text-alt text-error mt-1">{errors.display_options.message}</span>}
      </div>

      <div className="form-control flex-row items-center gap-3">
        <label className="label cursor-pointer gap-2" htmlFor="edit-layer-enabled">
          <span className="label-text">Enabled</span>
          <input
            id="edit-layer-enabled"
            type="checkbox"
            className="toggle toggle-primary toggle-sm"
            {...register("enabled")}
          />
        </label>
      </div>

      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton text="Save changes" loadingText="Saving..." loading={isPending} disabled={!isDirty} />
      </div>
    </form>
  );
};

export const EditLayerForm = ({
  layer,
  onClose,
}: {
  layer: DataProviderLayer | null;
  onClose: () => void;
}) => (
  <Modal id={MODAL_ID} title="Edit layer" onClose={onClose}>
    <EditLayerInner layer={layer} onClose={onClose} />
  </Modal>
);

export const openEditLayerModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
