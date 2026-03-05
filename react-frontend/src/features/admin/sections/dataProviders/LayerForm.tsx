import { useState } from "react";
import type { FieldErrors, UseFormRegister } from "react-hook-form";
import type { DataProviderServiceType, LayerCategory } from "@/domain/data_provider/types";
import { useArcGISServiceInfo, useArcGISFeatureServerLayers } from "@/hooks/useArcGISServiceInfo";

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
  arcgis_service_name: "",
  arcgis_layer_id: "",
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

function ArcGISPicker({
  baseUrl,
  register,
  errors,
}: {
  baseUrl: string | null;
  register: UseFormRegister<LayerFormData>;
  errors: FieldErrors<LayerFormData>;
}) {
  const [selectedServiceName, setSelectedServiceName] = useState("");

  const { data: services, isLoading: servicesLoading } = useArcGISServiceInfo(baseUrl ?? "");

  const featureServerUrl =
    baseUrl && selectedServiceName
      ? `${baseUrl}/${selectedServiceName}/FeatureServer`
      : "";
  const { data: layers, isLoading: layersLoading } =
    useArcGISFeatureServerLayers(featureServerUrl);

  return (
    <>
      <fieldset className="fieldset">
        <legend className="fieldset-legend">
          Service
          {servicesLoading && <span className="text-base-content/50 font-normal ml-1">Loading…</span>}
        </legend>
        {services && services.length > 0 ? (
          <select
            className={`select w-full ${errors.arcgis_service_name ? "select-error" : ""}`}
            {...register("arcgis_service_name", { required: "Service is required" })}
            onChange={(e) => setSelectedServiceName(e.target.value)}
          >
            <option value="">Select a service…</option>
            {services
              .filter((s) => s.type === "FeatureServer")
              .map((s) => (
                <option key={s.name} value={s.name}>{s.name}</option>
              ))}
          </select>
        ) : (
          <input
            type="text"
            placeholder="ServiceName"
            className={`input w-full ${errors.arcgis_service_name ? "input-error" : ""}`}
            {...register("arcgis_service_name", { required: "Service name is required" })}
            onChange={(e) => setSelectedServiceName(e.target.value)}
          />
        )}
        {errors.arcgis_service_name && (
          <p className="label text-error">{errors.arcgis_service_name.message}</p>
        )}
      </fieldset>

      <fieldset className="fieldset">
        <legend className="fieldset-legend">
          Layer
          {layersLoading && <span className="text-base-content/50 font-normal ml-1">Loading…</span>}
        </legend>
        {layers && layers.length > 0 ? (
          <select
            className={`select w-full ${errors.arcgis_layer_id ? "select-error" : ""}`}
            {...register("arcgis_layer_id", { required: "Layer is required" })}
          >
            <option value="">Select a layer…</option>
            {layers.map((l) => (
              <option key={l.id} value={l.id}>{l.id}: {l.name}</option>
            ))}
          </select>
        ) : (
          <input
            type="number"
            min={0}
            placeholder="0"
            className={`input w-full ${errors.arcgis_layer_id ? "input-error" : ""}`}
            {...register("arcgis_layer_id", { required: "Layer ID is required" })}
          />
        )}
        {errors.arcgis_layer_id && (
          <p className="label text-error">{errors.arcgis_layer_id.message}</p>
        )}
      </fieldset>
    </>
  );
}

interface LayerFormProps {
  register: UseFormRegister<LayerFormData>;
  errors: FieldErrors<LayerFormData>;
  serviceType: DataProviderServiceType | "";
  serviceBaseUrl?: string | null; // for ArcGIS layer picker
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
      <div className="grid grid-cols-2 gap-3">
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Name</legend>
          <input
            type="text"
            placeholder={mode === "create" ? "e.g. SSSI England" : undefined}
            className={`input w-full ${errors.name ? "input-error" : ""}`}
            {...register("name", { required: "Name is required" })}
          />
          {errors.name && <p className="label text-error">{errors.name.message}</p>}
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
        <input type="text" className="input w-full" {...register("description")} />
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
          <p className="label">{optionalLabel}</p>
        </fieldset>
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Subdivision</legend>
          <input
            type="text"
            placeholder="e.g. GB-ENG"
            className="input w-full"
            {...register("subdivision")}
          />
          <p className="label">{optionalLabel}</p>
        </fieldset>
      </div>

      {isMVT && (
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Tile URL</legend>
          <input
            type="text"
            placeholder="/api/tiles/{workspace}/{layer}/{z}/{x}/{-y}"
            className={`input w-full font-mono text-sm ${errors.mvt_url ? "input-error" : ""}`}
            {...register("mvt_url", { required: "Tile URL is required" })}
          />
          {errors.mvt_url && <p className="label text-error">{errors.mvt_url.message}</p>}
        </fieldset>
      )}

      {isArcGIS && (
        <ArcGISPicker
          baseUrl={serviceBaseUrl ?? null}
          register={register}
          errors={errors}
        />
      )}

      {!isMVT && !isArcGIS && (
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Source (JSON)</legend>
          <textarea
            rows={3}
            className={`textarea w-full font-mono text-xs ${errors.source ? "textarea-error" : ""}`}
            {...register("source", { required: "Source is required" })}
          />
          {errors.source && <p className="label text-error">{errors.source.message}</p>}
        </fieldset>
      )}

      <fieldset className="fieldset">
        <legend className="fieldset-legend">Style config (JSON)</legend>
        <textarea
          rows={2}
          className={`textarea w-full font-mono text-xs ${errors.style_config ? "textarea-error" : ""}`}
          {...register("style_config")}
        />
        {errors.style_config
          ? <p className="label text-error">{errors.style_config.message}</p>
          : <p className="label">{optionalLabel}</p>
        }
      </fieldset>

      <fieldset className="fieldset">
        <legend className="fieldset-legend">Display options (JSON)</legend>
        <textarea
          rows={2}
          className={`textarea w-full font-mono text-xs ${errors.display_options ? "textarea-error" : ""}`}
          {...register("display_options")}
        />
        {errors.display_options
          ? <p className="label text-error">{errors.display_options.message}</p>
          : <p className="label">{optionalLabel}</p>
        }
      </fieldset>
    </>
  );
};
