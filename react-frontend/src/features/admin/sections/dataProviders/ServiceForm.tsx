import { useState, useEffect } from "react";
import type { FieldErrors, UseFormRegister } from "react-hook-form";
import type { DataProviderServiceType } from "@/domain/data_provider/types";
import { useArcGISServiceInfo } from "@/hooks/useArcGISServiceInfo";

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

const SERVICE_TYPES = Object.keys(
  SERVICE_TYPE_LABELS,
) as DataProviderServiceType[];

function useDebounce(value: string, delay: number) {
  const [debounced, setDebounced] = useState(value);
  useEffect(() => {
    const t = setTimeout(() => setDebounced(value), delay);
    return () => clearTimeout(t);
  }, [value, delay]);
  return debounced;
}

function ArcGISHealthCheck({ baseUrl }: { baseUrl: string }) {
  const { data: services, isLoading, isError } = useArcGISServiceInfo(baseUrl);

  if (!baseUrl) return null;
  if (isLoading) return <p className="label">Checking service…</p>;
  if (isError)
    return <p className="label text-error">Could not reach service</p>;
  if (services) {
    return (
      <p className="label text-success flex items-center gap-1">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2.5"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <polyline points="20 6 9 17 4 12" />
        </svg>
        {services.length} service{services.length !== 1 ? "s" : ""} available
      </p>
    );
  }
  return null;
}

interface ServiceFormProps {
  register: UseFormRegister<ServiceFormData>;
  errors: FieldErrors<ServiceFormData>;
  needsBaseUrl: boolean;
  mode: "create" | "edit";
  serviceType: DataProviderServiceType | "";
  baseUrl: string;
}

export const ServiceForm = ({
  register,
  errors,
  needsBaseUrl,
  mode,
  serviceType,
  baseUrl,
}: ServiceFormProps) => {
  const debouncedBaseUrl = useDebounce(baseUrl, 500);

  return (
    <>
      <fieldset className="fieldset">
        <legend className="fieldset-legend">Service type</legend>
        <select
          className={`select w-full ${errors.service_type ? "select-error" : ""}`}
          {...register("service_type", {
            required: "Service type is required",
          })}
        >
          <option value="" disabled>
            Select a service type…
          </option>
          {SERVICE_TYPES.map((t) => (
            <option key={t} value={t}>
              {SERVICE_TYPE_LABELS[t]}
            </option>
          ))}
        </select>
        {errors.service_type && (
          <p className="label text-error">{errors.service_type.message}</p>
        )}
      </fieldset>

      <fieldset className="fieldset">
        <legend className="fieldset-legend">Name</legend>
        <input
          type="text"
          placeholder={
            mode === "create" ? "e.g. Natural England WMS" : undefined
          }
          className={`input w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Name is required" })}
        />
        {errors.name && (
          <p className="label text-error">{errors.name.message}</p>
        )}
      </fieldset>

      {needsBaseUrl && (
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Base URL</legend>
          <input
            type="text"
            placeholder="https://..."
            className={`input w-full ${errors.base_url ? "input-error" : ""}`}
            {...register("base_url")}
          />
          {errors.base_url && (
            <p className="label text-error">{errors.base_url.message}</p>
          )}
          {serviceType === "ArcGISRest" && (
            <ArcGISHealthCheck baseUrl={debouncedBaseUrl} />
          )}
        </fieldset>
      )}
    </>
  );
};
