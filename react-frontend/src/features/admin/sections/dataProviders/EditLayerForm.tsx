import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchDataProviderLayer } from "@/hooks/api/usePatchDataProviderLayer";
import { useDataProviderServices } from "@/hooks/api/useDataProviderServices";
import { ApiError } from "@/lib/api";
import type { DataProviderLayer } from "@/domain/data_provider/types";
import {
  LayerForm,
  LAYER_FORM_DEFAULTS,
  buildStyleConfig,
  parseStyleConfig,
  buildDisplayOptions,
  parseDisplayOptions,
  type LayerFormData,
} from "./LayerForm";

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

  const mvtUrl =
    isMVT &&
    typeof layer?.source === "object" &&
    layer.source !== null &&
    "url" in layer.source
      ? String((layer.source as { url: string }).url)
      : "";

  const arcgisSrc =
    isArcGIS && typeof layer?.source === "object" && layer.source !== null
      ? (layer.source as { service_name?: string; layer_id?: number })
      : null;
  const arcgisServiceName = arcgisSrc?.service_name ?? "";
  const arcgisLayerId =
    arcgisSrc?.layer_id != null ? String(arcgisSrc.layer_id) : "";

  const {
    register,
    handleSubmit,
    watch,
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
          source:
            isMVT || isArcGIS
              ? "{}"
              : (JSON.stringify(layer.source, null, 2) ?? "{}"),
          category: layer.category,
          description: layer.description ?? "",
          enabled_geoman: layer.enabled_geoman,
          enabled_figure_tool: layer.enabled_figure_tool,
          ...parseStyleConfig(layer.style_config),
          ...parseDisplayOptions(layer.display_options),
          country_code: layer.country_code ?? "",
          subdivision: layer.subdivision ?? "",
          sort_order: String(layer.sort_order),
        }
      : undefined,
  });

  const styleEnabled = watch("style_enabled");

  const onSubmit = (data: LayerFormData) => {
    if (!layer) return;

    const patch: Parameters<typeof patchLayer>[0]["patch"] = {};

    if (dirtyFields.name) patch.name = data.name;
    if (dirtyFields.abbreviation)
      patch.abbreviation = data.abbreviation || null;
    if (dirtyFields.category) patch.category = data.category;
    if (dirtyFields.description) patch.description = data.description || null;
    if (dirtyFields.enabled_geoman) patch.enabled_geoman = data.enabled_geoman;
    if (dirtyFields.enabled_figure_tool)
      patch.enabled_figure_tool = data.enabled_figure_tool;
    if (dirtyFields.country_code)
      patch.country_code = data.country_code || null;
    if (dirtyFields.subdivision) patch.subdivision = data.subdivision || null;
    if (dirtyFields.sort_order)
      patch.sort_order = data.sort_order ? Number(data.sort_order) : undefined;

    if (isMVT && dirtyFields.mvt_url) {
      patch.source = { url: data.mvt_url };
    } else if (
      isArcGIS &&
      (dirtyFields.arcgis_service_name || dirtyFields.arcgis_layer_id)
    ) {
      patch.source = {
        service_name: data.arcgis_service_name,
        layer_id: Number(data.arcgis_layer_id),
      };
    } else if (!isMVT && !isArcGIS && dirtyFields.source) {
      try {
        patch.source = JSON.parse(data.source);
      } catch {
        setError("source", { message: "Must be valid JSON" });
        return;
      }
    }

    const styleFieldsDirty =
      dirtyFields.style_enabled || dirtyFields.style_color;
    if (styleFieldsDirty) {
      patch.style_config = buildStyleConfig(data) ?? null;
    }

    if (dirtyFields.min_zoom || dirtyFields.max_zoom) {
      patch.display_options = buildDisplayOptions(data) ?? null;
    }

    patchLayer(
      { id: layer.id, patch },
      {
        onSuccess: () => {
          closeDialog();
          onClose();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to update layer: ${message}`);
        },
      },
    );
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <LayerForm
        register={register}
        errors={errors}
        serviceType={serviceType}
        serviceBaseUrl={service?.base_url}
        sourceComplete={true}
        styleEnabled={styleEnabled}
        mode="edit"
      />
      <div className="modal-action">
        <CancelButton
          onClick={() => {
            closeDialog();
            onClose();
          }}
          disabled={isPending}
        />
        <SubmitButton
          text="Save changes"
          loadingText="Saving..."
          loading={isPending}
          disabled={!isDirty}
        />
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
