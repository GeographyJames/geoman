import { useState } from "react";
import type { FieldErrors, UseFormRegister } from "react-hook-form";

import { useArcGISServiceInfo } from "@/hooks/useArcGISServiceInfo";
import type { LayerFormData } from "../LayerForm";
import SelectLayer from "./SelectLayers";

export function ArcGISPicker({
  baseUrl,
  register,
  errors,
}: {
  baseUrl: string | null;
  register: UseFormRegister<LayerFormData>;
  errors: FieldErrors<LayerFormData>;
}) {
  const [selectedServiceName, setSelectedServiceName] = useState("");

  const {
    data: services,
    isError: servicesError,
    isPending: servicesPending,
  } = useArcGISServiceInfo(baseUrl ?? "");

  const featureServerUrl =
    baseUrl && selectedServiceName
      ? `${baseUrl}/${selectedServiceName}/FeatureServer`
      : "";

  return (
    <>
      <fieldset className="fieldset">
        <legend className="fieldset-legend">ArcGIS Rest API Service</legend>

        <select
          className={`select w-full ${errors.arcgis_service_name ? "select-error" : ""}`}
          {...register("arcgis_service_name", {
            required: "Service is required",
          })}
          onChange={(e) => setSelectedServiceName(e.target.value)}
        >
          {servicesPending ? (
            <option value="" disabled>
              "loading services..."{" "}
            </option>
          ) : servicesError ? (
            <option value="" disabled>
              no services returned from server
            </option>
          ) : (
            <>
              <option value="" disabled>
                select service
              </option>
              {services
                .filter((s) => s.type === "FeatureServer")
                .map((s) => (
                  <option key={s.name} value={s.name}>
                    {s.name}
                  </option>
                ))}
            </>
          )}
        </select>

        {errors.arcgis_service_name && (
          <p className="label text-error">
            {errors.arcgis_service_name.message}
          </p>
        )}
      </fieldset>

      {selectedServiceName && (
        <SelectLayer
          selectedServiceName={selectedServiceName}
          baseUrl={baseUrl}
          register={register}
          errors={errors}
        />
      )}
    </>
  );
}
