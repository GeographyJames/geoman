import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchDataProviderLayer } from "@/hooks/api/usePatchDataProviderLayer";
import { useDataProviderServices } from "@/hooks/api/useDataProviderServices";
import { ApiError } from "@/lib/api";
import type { DataProviderLayer } from "@/domain/data_provider/types";
import { LayerForm, LAYER_FORM_DEFAULTS, jsonFieldValue, type LayerFormData } from "./LayerForm";

const MODAL_ID = "edit_data_provider_layer";

const EditLayerInner = ({
  layer,
  onClose,
}: {
  layer: DataProviderLayer | null;
  onClose: () => void;
}) => {
  const { mutate: patchLayer, isPending } = usePatchDataProviderLayer();
  const { data: services = [] } = useDataProviderServices();
  const { addError, closeDialog } = useModal();

  const service = services.find((s) => s.id === layer?.service_id);
  const serviceType = service?.service_type ?? "";
  const isMVT = serviceType === "MVT";
  const isArcGIS = serviceType === "ArcGISRest";

  const mvtUrl = isMVT && typeof layer?.source === "object" && layer.source !== null && "url" in layer.source
    ? String((layer.source as { url: string }).url)
    : "";

  const arcgisSrc = isArcGIS && typeof layer?.source === "object" && layer.source !== null
    ? layer.source as { service_name?: string; layer_id?: number }
    : null;
  const arcgisServiceName = arcgisSrc?.service_name ?? "";
  const arcgisLayerId = arcgisSrc?.layer_id != null ? String(arcgisSrc.layer_id) : "";

  const {
    register,
    handleSubmit,
    formState: { errors, isDirty, dirtyFields },
    setError,
  } = useForm<LayerFormData>({
    values: layer
      ? {
          ...LAYER_FORM_DEFAULTS,
          name: layer.name,
          abbreviation: layer.abbreviation ?? "",
          mvt_url: mvtUrl,
          arcgis_service_name: arcgisServiceName,
          arcgis_layer_id: arcgisLayerId,
          source: (isMVT || isArcGIS) ? "{}" : (JSON.stringify(layer.source, null, 2) ?? "{}"),
          category: layer.category,
          description: layer.description ?? "",
          enabled: layer.enabled,
          style_config: jsonFieldValue(layer.style_config),
          display_options: jsonFieldValue(layer.display_options),
          country_code: layer.country_code ?? "",
          subdivision: layer.subdivision ?? "",
          sort_order: String(layer.sort_order),
        }
      : undefined,
  });

  const onSubmit = (data: LayerFormData) => {
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

    if (isMVT && dirtyFields.mvt_url) {
      patch.source = { url: data.mvt_url };
    } else if (isArcGIS && (dirtyFields.arcgis_service_name || dirtyFields.arcgis_layer_id)) {
      patch.source = { service_name: data.arcgis_service_name, layer_id: Number(data.arcgis_layer_id) };
    } else if (!isMVT && !isArcGIS && dirtyFields.source) {
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

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <LayerForm register={register} errors={errors} serviceType={serviceType} serviceBaseUrl={service?.base_url} mode="edit" />

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
        <CancelButton onClick={() => { closeDialog(); onClose(); }} disabled={isPending} />
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
