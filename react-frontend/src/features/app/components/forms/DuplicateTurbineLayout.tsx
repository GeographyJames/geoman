import { useEffect, useState } from "react";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { TextInput } from "@/components/forms/components/TextInput";
import { useDuplicateTurbineLayoutContext } from "@/features/app/contexts/DuplicateTurbineLayoutContext";
import { useDuplicateTurbineLayout } from "@/hooks/api/projectFeature.ts/useDuplicateTurbineLayout";
import { ApiError } from "@/lib/api";

const DuplicateTurbineLayoutInner = () => {
    const { feature, clear } = useDuplicateTurbineLayoutContext();
    const { mutate: duplicateLayout, isPending } = useDuplicateTurbineLayout();
    const { addError, closeDialog } = useModal();

    const [name, setName] = useState("");
    const [hubHeight, setHubHeight] = useState("");
    const [rotorDiameter, setRotorDiameter] = useState("");
    const [primary, setPrimary] = useState(false);

    useEffect(() => {
        if (feature) {
            setName(feature.properties.name);
            setHubHeight(
                feature.properties.hub_height_mm != null
                    ? String(feature.properties.hub_height_mm / 1000)
                    : "",
            );
            setRotorDiameter(
                feature.properties.rotor_diameter_mm != null
                    ? String(feature.properties.rotor_diameter_mm / 1000)
                    : "",
            );
            setPrimary(false);
        }
    }, [feature]);

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        if (!feature) return;
        duplicateLayout(
            {
                projectId: feature.properties.project_id,
                collectionId: feature.properties.collection_id,
                id: feature.id,
                dto: {
                    name: name || undefined,
                    hub_height_metre: hubHeight ? Number(hubHeight) : undefined,
                    rotor_diameter_metre: rotorDiameter ? Number(rotorDiameter) : undefined,
                    primary: primary || undefined,
                },
            },
            {
                onSuccess: () => {
                    closeDialog();
                    clear();
                },
                onError: (error) => {
                    const message =
                        error instanceof ApiError && error.status === 500
                            ? "internal server error"
                            : error.message;
                    addError(`Unable to duplicate layout: ${message}`);
                },
            },
        );
    };

    const handleCancel = () => {
        closeDialog();
        clear();
    };

    return (
        <form onSubmit={handleSubmit} className="space-y-4">
            {feature && (
                <>
                    <TextInput
                        name="name"
                        label="Name"
                        required
                        value={name}
                        onChange={setName}
                    />
                    <div className="flex gap-4">
                        <fieldset className="fieldset flex-1">
                            <legend className="fieldset-legend">Hub height (m)</legend>
                            <input
                                type="number"
                                className="input input-bordered w-full"
                                value={hubHeight}
                                onChange={(e) => setHubHeight(e.target.value)}
                                min="0"
                                step="0.1"
                            />
                        </fieldset>
                        <fieldset className="fieldset flex-1">
                            <legend className="fieldset-legend">Rotor diameter (m)</legend>
                            <input
                                type="number"
                                className="input input-bordered w-full"
                                value={rotorDiameter}
                                onChange={(e) => setRotorDiameter(e.target.value)}
                                min="0"
                                step="0.1"
                            />
                        </fieldset>
                    </div>
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
            <div className="modal-action">
                <CancelButton onClick={handleCancel} disabled={isPending} />
                <SubmitButton
                    text="Duplicate"
                    loadingText="Duplicating..."
                    loading={isPending}
                />
            </div>
        </form>
    );
};

export const DuplicateTurbineLayoutForm = () => {
    return (
        <Modal
            id="duplicate_turbine_layout"
            title="Duplicate turbine layout"
            onClose={useDuplicateTurbineLayoutContext().clear}
        >
            <DuplicateTurbineLayoutInner />
        </Modal>
    );
};
