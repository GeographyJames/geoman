import type { FieldErrors, UseFormRegister } from "react-hook-form";
import type { LayerCategory } from "@/domain/data_provider/types";

export interface LayerFormData {
  service_id: string; // create only — provider/service dropdowns in CreateLayerForm
  name: string;
  abbreviation: string;
  mvt_url: string; // MVT source URL
  source: string; // non-MVT source JSON
  category: LayerCategory;
  description: string;
  style_config: string;
  display_options: string;
  country_code: string;
  subdivision: string;
  sort_order: string;
  enabled: boolean; // edit only — enabled toggle in EditLayerForm
}

export const LAYER_FORM_DEFAULTS: LayerFormData = {
  service_id: "",
  name: "",
  abbreviation: "",
  mvt_url: "",
  source: "{}",
  category: "overlay",
  description: "",
  style_config: "",
  display_options: "",
  country_code: "",
  subdivision: "",
  sort_order: "",
  enabled: true,
};

/** Serialises a JSONB value for display in a textarea — returns empty string for null/empty objects. */
export function jsonFieldValue(v: unknown): string {
  if (!v || (typeof v === "object" && Object.keys(v as object).length === 0))
    return "";
  return JSON.stringify(v, null, 2);
}

interface LayerFormProps {
  register: UseFormRegister<LayerFormData>;
  errors: FieldErrors<LayerFormData>;
  isMVT: boolean;
  mode: "create" | "edit";
}

export const LayerForm = ({
  register,
  errors,
  isMVT,
  mode,
}: LayerFormProps) => {
  const optional =
    mode === "create" ? (
      <span className="label-text-alt text-base-content/50">optional</span>
    ) : (
      <span className="label-text-alt text-base-content/50">
        clear to remove
      </span>
    );

  return (
    <>
      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="layer-name">
            <span className="label-text">Name</span>
          </label>
          <input
            id="layer-name"
            type="text"
            placeholder={mode === "create" ? "e.g. SSSI England" : undefined}
            className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
            {...register("name", { required: "Name is required" })}
          />
          {errors.name && (
            <span className="label-text-alt text-error mt-1">
              {errors.name.message}
            </span>
          )}
        </div>

        <div className="form-control">
          <label className="label" htmlFor="layer-abbr">
            <span className="label-text">Abbreviation</span>
            {optional}
          </label>
          <input
            id="layer-abbr"
            type="text"
            placeholder={mode === "create" ? "e.g. SSSI" : undefined}
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
            {optional}
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
          {optional}
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
            {optional}
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
            {optional}
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

      {isMVT ? (
        <div className="form-control">
          <label className="label" htmlFor="layer-mvt-url">
            <span className="label-text">Tile URL</span>
          </label>
          <input
            id="layer-mvt-url"
            type="text"
            placeholder="/api/tiles/{workspace}/{layer}/{z}/{x}/{-y}"
            className={`input input-bordered w-full font-mono text-sm ${errors.mvt_url ? "input-error" : ""}`}
            {...register("mvt_url", { required: "Tile URL is required" })}
          />
          {errors.mvt_url && (
            <span className="label-text-alt text-error mt-1">
              {errors.mvt_url.message}
            </span>
          )}
        </div>
      ) : (
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
          {errors.source && (
            <span className="label-text-alt text-error mt-1">
              {errors.source.message}
            </span>
          )}
        </div>
      )}

      <div className="form-control">
        <label className="label" htmlFor="layer-style">
          <span className="label-text">Style config (JSON)</span>
          {optional}
        </label>
        <textarea
          id="layer-style"
          rows={2}
          className={`textarea textarea-bordered w-full font-mono text-xs ${errors.style_config ? "textarea-error" : ""}`}
          {...register("style_config")}
        />
        {errors.style_config && (
          <span className="label-text-alt text-error mt-1">
            {errors.style_config.message}
          </span>
        )}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="layer-display-options">
          <span className="label-text">Display options (JSON)</span>
          {optional}
        </label>
        <textarea
          id="layer-display-options"
          rows={2}
          className={`textarea textarea-bordered w-full font-mono text-xs ${errors.display_options ? "textarea-error" : ""}`}
          {...register("display_options")}
        />
        {errors.display_options && (
          <span className="label-text-alt text-error mt-1">
            {errors.display_options.message}
          </span>
        )}
      </div>
    </>
  );
};
