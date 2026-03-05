import type { FieldErrors, UseFormRegister } from "react-hook-form";
import { useArcGISFeatureServerLayers } from "@/hooks/useArcGISServiceInfo";
import type { LayerFormData } from "../LayerForm";

export default function SelectLayer({
  selectedServiceName,
  baseUrl,
  register,
  errors,
}: {
  selectedServiceName: string;
  baseUrl: string | null;
  register: UseFormRegister<LayerFormData>;
  errors: FieldErrors<LayerFormData>;
}) {
  const featureServerUrl =
    baseUrl && selectedServiceName
      ? `${baseUrl}/${selectedServiceName}/FeatureServer`
      : "";

  const {
    data: layers,
    isPending: layersPending,
    isError: layersError,
  } = useArcGISFeatureServerLayers(featureServerUrl);

  return (
    <fieldset className="fieldset">
      <legend className="fieldset-legend">Layer</legend>

      <select
        className={`select w-full ${errors.arcgis_layer_id ? "select-error" : ""}`}
        {...register("arcgis_layer_id", { required: "Layer is required" })}
      >
        {layersPending ? (
          <option value="" disabled>loading layers...</option>
        ) : layersError ? (
          <option value="" disabled>no layers returned from server</option>
        ) : (
          <>
            <option value="" disabled>select layer</option>
            {layers.map((l) => (
              <option key={l.id} value={l.id}>
                {l.id}: {l.name}
              </option>
            ))}
          </>
        )}
      </select>

      {errors.arcgis_layer_id && (
        <p className="label text-error">{errors.arcgis_layer_id.message}</p>
      )}
    </fieldset>
  );
}
