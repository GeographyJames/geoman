import type { FieldErrors, UseFormRegister } from "react-hook-form";
import type {
  DataProviderServiceType,
  LayerCategory,
} from "@/domain/data_provider/types";
import { ArcGISPicker } from "./ArcGISRest/ArcGISPicker";

export interface LayerFormData {
  service_id: string; // create only — provider/service dropdowns in CreateLayerForm
  name: string;
  abbreviation: string;
  mvt_url: string; // MVT source URL
  arcgis_service_name: string; // ArcGISRest service name within the directory
  arcgis_layer_id: string; // ArcGISRest layer index within the FeatureServer
  source: string; // other source types — raw JSON
  category: LayerCategory;
  description: string;
  style_color: string;
  min_zoom: string;
  max_zoom: string;
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
  arcgis_service_name: "",
  arcgis_layer_id: "",
  source: "{}",
  category: "overlay",
  description: "",
  style_color: "#3388ff",
  min_zoom: "",
  max_zoom: "",
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

type StyleFields = Pick<LayerFormData, "style_color">;

/** Builds the style_config JSONB value from the colour picker fields, or undefined if style is disabled. */
export function buildStyleConfig(data: StyleFields): object | undefined {
  return {
    fillColor: data.style_color,
    fillOpacity: 0.2,
    strokeColor: data.style_color,
    strokeWidth: 2,
  };
}

/** Parses a stored style_config JSONB value back into colour picker form fields. */
export function parseStyleConfig(styleConfig: unknown): StyleFields {
  const defaults: StyleFields = {
    style_color: "#3388ff",
  };
  if (!styleConfig || typeof styleConfig !== "object") return defaults;
  const cfg = styleConfig as Record<string, unknown>;
  const color = typeof cfg.fillColor === "string" ? cfg.fillColor : null;
  if (!color) return defaults;
  return { style_color: color };
}

type ZoomFields = Pick<LayerFormData, "min_zoom" | "max_zoom">;

/** Builds the display_options JSONB value from zoom fields, or undefined if neither is set. */
export function buildDisplayOptions(data: ZoomFields): object | undefined {
  const minZoom = data.min_zoom ? Number(data.min_zoom) : undefined;
  const maxZoom = data.max_zoom ? Number(data.max_zoom) : undefined;
  if (minZoom === undefined && maxZoom === undefined) return undefined;
  return {
    ...(minZoom !== undefined && { minZoom }),
    ...(maxZoom !== undefined && { maxZoom }),
  };
}

/** Parses a stored display_options JSONB value back into zoom form fields. */
export function parseDisplayOptions(displayOptions: unknown): ZoomFields {
  const defaults: ZoomFields = { min_zoom: "", max_zoom: "" };
  if (!displayOptions || typeof displayOptions !== "object") return defaults;
  const cfg = displayOptions as Record<string, unknown>;
  return {
    min_zoom: typeof cfg.minZoom === "number" ? String(cfg.minZoom) : "",
    max_zoom: typeof cfg.maxZoom === "number" ? String(cfg.maxZoom) : "",
  };
}

const ZOOM_OPTIONS = Array.from({ length: 20 }, (_, i) => i + 1);

interface LayerFormProps {
  register: UseFormRegister<LayerFormData>;
  errors: FieldErrors<LayerFormData>;
  serviceType: DataProviderServiceType | "";
  serviceBaseUrl?: string | null;
  sourceComplete: boolean;

  mode: "create" | "edit";
}

export const LayerForm = ({
  register,
  errors,
  serviceType,
  serviceBaseUrl,
  mode,
}: LayerFormProps) => {
  const isMVT = serviceType === "MVT";
  const isArcGIS = serviceType === "ArcGISRest";
  const optionalLabel = mode === "create" ? "optional" : "clear to remove";

  return (
    <>
      {isArcGIS && mode === "create" && (
        <ArcGISPicker
          baseUrl={serviceBaseUrl ?? null}
          register={register}
          errors={errors}
        />
      )}

      {isMVT && mode === "create" && (
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Tile URL</legend>
          <input
            type="text"
            placeholder="/api/tiles/{workspace}/{layer}/{z}/{x}/{-y}"
            className={`input w-full font-mono text-sm ${errors.mvt_url ? "input-error" : ""}`}
            {...register("mvt_url", { required: "Tile URL is required" })}
          />
          {errors.mvt_url && (
            <p className="label text-error">{errors.mvt_url.message}</p>
          )}
        </fieldset>
      )}

      {!isMVT && !isArcGIS && (
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Source (JSON)</legend>
          <textarea
            rows={3}
            className={`textarea w-full font-mono text-xs ${errors.source ? "textarea-error" : ""}`}
            {...register("source", { required: "Source is required" })}
          />
          {errors.source && (
            <p className="label text-error">{errors.source.message}</p>
          )}
        </fieldset>
      )}

      <>
        <div className="grid grid-cols-2 gap-3">
          <fieldset className="fieldset">
            <legend className="fieldset-legend">Layer name</legend>
            <input
              type="text"
              placeholder={mode === "create" ? "e.g. SSSI England" : undefined}
              className={`input w-full ${errors.name ? "input-error" : ""}`}
              {...register("name", { required: "Name is required" })}
            />
            {errors.name && (
              <p className="label text-error">{errors.name.message}</p>
            )}
          </fieldset>

          <fieldset className="fieldset">
            <legend className="fieldset-legend">Abbreviation</legend>
            <input
              type="text"
              placeholder={mode === "create" ? "e.g. SSSI" : undefined}
              className="input w-full"
              {...register("abbreviation")}
            />
            <p className="label">{optionalLabel}</p>
          </fieldset>
        </div>

        <div className="grid grid-cols-2 gap-3">
          <fieldset className="fieldset">
            <legend className="fieldset-legend">Category</legend>
            <select className="select w-full" {...register("category")}>
              <option value="overlay">Overlay</option>
              <option value="basemap">Base map</option>
            </select>
          </fieldset>

          <fieldset className="fieldset">
            <legend className="fieldset-legend">Sort order</legend>
            <input
              type="number"
              className="input w-full"
              {...register("sort_order")}
            />
            <p className="label">{optionalLabel}</p>
          </fieldset>
        </div>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Description</legend>
          <textarea className="textarea w-full" {...register("description")} />
          <p className="label">{optionalLabel}</p>
        </fieldset>

        <div className="grid grid-cols-2 gap-3">
          <fieldset className="fieldset">
            <legend className="fieldset-legend">Country code</legend>
            <input
              type="text"
              placeholder="e.g. GB"
              className="input w-full"
              {...register("country_code")}
            />
            <p className="label">optional (overides data provider)</p>
          </fieldset>
          <fieldset className="fieldset">
            <legend className="fieldset-legend">Subdivision</legend>
            <input
              type="text"
              placeholder="e.g. GB-ENG"
              className="input w-full"
              {...register("subdivision")}
            />
            <p className="label">optional (overides data provider)</p>
          </fieldset>
        </div>

        <fieldset className="fieldset">
          <legend className="fieldset-legend">Style</legend>

          <input
            type="color"
            className="mt-2 input w-full h-10 cursor-pointer p-1"
            {...register("style_color")}
          />
        </fieldset>

        <div className="grid grid-cols-2 gap-3">
          <fieldset className="fieldset">
            <legend className="fieldset-legend">Min zoom</legend>
            <select className="select w-full" {...register("min_zoom")}>
              <option value="">— (any)</option>
              {ZOOM_OPTIONS.map((z) => (
                <option key={z} value={z}>
                  z{z}
                </option>
              ))}
            </select>
          </fieldset>
          <fieldset className="fieldset">
            <legend className="fieldset-legend">Max zoom</legend>
            <select className="select w-full" {...register("max_zoom")}>
              <option value="">— (any)</option>
              {ZOOM_OPTIONS.map((z) => (
                <option key={z} value={z}>
                  z{z}
                </option>
              ))}
            </select>
          </fieldset>
        </div>
      </>
    </>
  );
};
