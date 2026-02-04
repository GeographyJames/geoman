import { useEffect, useState } from "react";
import { ModalForm } from "@/components/forms/ModalForm";
import { TextInput } from "@/components/forms/components/TextInput";
import { useEditFeature } from "@/features/app/contexts/EditFeatureContext";
import { usePatchProjectFeature } from "@/hooks/api/projectFeature.ts/usePatchProjectFeature";
import { useFlash } from "../../contexts/FlashMessageContext";

export const EditFeatureForm = () => {
  const { feature, clear } = useEditFeature();
  const { mutateAsync: patchFeature } = usePatchProjectFeature();
  const { addFlash } = useFlash();
  const [name, setName] = useState("");
  const [primary, setPrimary] = useState(false);

  useEffect(() => {
    if (feature) {
      setName(feature.properties.name);
      setPrimary(feature.properties.is_primary);
    }
  }, [feature]);

  const handleSubmit = async () => {
    if (!feature) return;
    await patchFeature(
      {
        projectId: feature.properties.project_id,
        collectionId: feature.properties.collection_id.toString(),
        id: feature.id,
        dto: { name, primary },
      },
      {
        onError: (error) => {
          addFlash(`Unable to update feature: ${error.message}`, "error");
        },
      },
    );
  };

  const handleReset = () => {
    setName(feature?.properties.name ?? "");
    setPrimary(feature?.properties.is_primary ?? false);
  };

  return (
    <ModalForm
      id="edit_feature"
      title="Edit feature"
      onSubmit={handleSubmit}
      onReset={handleReset}
      onClose={clear}
    >
      {feature && (
        <>
          <TextInput
            name="name"
            label="Name"
            required
            value={name}
            onChange={setName}
          />
          <label className="label cursor-pointer justify-start gap-2">
            <input
              type="checkbox"
              className="checkbox"
              checked={primary}
              onChange={(e) => setPrimary(e.target.checked)}
            />
            <span className="label-text text-base-content">Set as primary</span>
          </label>
        </>
      )}
    </ModalForm>
  );
};
