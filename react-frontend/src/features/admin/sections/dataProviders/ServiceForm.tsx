import type { FieldErrors, UseFormRegister } from "react-hook-form";
import type { DataProviderServiceType } from "@/domain/data_provider/types";

export interface ServiceFormData {
  name: string;
  service_type: DataProviderServiceType | "";
  base_url: string;
}

export const SERVICE_FORM_DEFAULTS: ServiceFormData = {
  name: "",
  service_type: "",
  base_url: "",
};

const SERVICE_TYPE_LABELS: Record<DataProviderServiceType, string> = {
  ImageWMS: "ImageWMS — Image Web Map Service",
  TileWMS: "TileWMS — Tiled Web Map Service",
  WMTS: "WMTS — Web Map Tile Service",
  WFS: "WFS — Web Feature Service",
  ArcGISRest: "ArcGISRest — ArcGIS REST Feature Service",
  MVT: "MVT — Mapbox Vector Tiles",
  OGCAPIFeatures: "OGCAPIFeatures — OGC API Features",
  XYZ: "XYZ — XYZ Tile Layer",
};

const SERVICE_TYPES = Object.keys(SERVICE_TYPE_LABELS) as DataProviderServiceType[];

interface ServiceFormProps {
  register: UseFormRegister<ServiceFormData>;
  errors: FieldErrors<ServiceFormData>;
  needsBaseUrl: boolean;
  mode: "create" | "edit";
}

export const ServiceForm = ({
  register,
  errors,
  needsBaseUrl,
  mode,
}: ServiceFormProps) => (
  <>
    <div className="form-control">
      <label className="label" htmlFor="service-type">
        <span className="label-text">Service type</span>
      </label>
      <select
        id="service-type"
        className={`select select-bordered w-full ${errors.service_type ? "select-error" : ""}`}
        {...register("service_type", { required: "Service type is required" })}
      >
        <option value="" disabled>Select a service type…</option>
        {SERVICE_TYPES.map((t) => (
          <option key={t} value={t}>
            {SERVICE_TYPE_LABELS[t]}
          </option>
        ))}
      </select>
      {errors.service_type && (
        <span className="label-text-alt text-error mt-1">{errors.service_type.message}</span>
      )}
    </div>

    <div className="form-control">
      <label className="label" htmlFor="service-name">
        <span className="label-text">Name</span>
      </label>
      <input
        id="service-name"
        type="text"
        placeholder={mode === "create" ? "e.g. Natural England WMS" : undefined}
        className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
        {...register("name", { required: "Name is required" })}
      />
      {errors.name && (
        <span className="label-text-alt text-error mt-1">
          {errors.name.message}
        </span>
      )}
    </div>

    {needsBaseUrl && (
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
        {errors.base_url && (
          <span className="label-text-alt text-error mt-1">
            {errors.base_url.message}
          </span>
        )}
      </div>
    )}
  </>
);
