import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchTeam } from "@/hooks/api/usePatchTeam";
import { useBusinessUnits } from "@/hooks/api/useBusinessUnits";
import { ApiError } from "@/lib/api";
import type Team from "@/domain/team/entity";

interface EditTeamFormData {
  name: string;
  businessUnitId: number | "";
}

const MODAL_ID = "edit_team";

const EditTeamInner = ({ team, onClose }: { team: Team | null; onClose: () => void }) => {
  const { mutate: patchTeam, isPending } = usePatchTeam();
  const { data: businessUnits = [] } = useBusinessUnits();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    formState: { errors, isDirty, dirtyFields },
  } = useForm<EditTeamFormData>({
    values: team
      ? { name: team.name, businessUnitId: team.businessUnitId ?? "" }
      : undefined,
  });

  const onSubmit = (data: EditTeamFormData) => {
    if (!team) return;
    patchTeam(
      {
        teamId: team.id,
        patch: {
          name: dirtyFields.name ? data.name : undefined,
          business_unit: dirtyFields.businessUnitId ? (data.businessUnitId ? Number(data.businessUnitId) : null) : undefined,
        },
      },
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
          addError(`Unable to update team: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    closeDialog();
    onClose();
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Team name</legend>
        <input
          type="text"
          className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Team name is required" })}
        />
        {errors.name && (
          <p className="label text-error">{errors.name.message}</p>
        )}
      </fieldset>

      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Business unit</legend>
        <select
          className="select select-bordered w-full"
          {...register("businessUnitId")}
        >
          <option value="">None</option>
          {businessUnits.map((bu) => (
            <option key={bu.id} value={bu.id}>
              {bu.name}
            </option>
          ))}
        </select>
      </fieldset>

      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
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

export const EditTeamForm = ({ team, onClose }: { team: Team | null; onClose: () => void }) => (
  <Modal id={MODAL_ID} title="Edit team" onClose={onClose}>
    <EditTeamInner team={team} onClose={onClose} />
  </Modal>
);

export const openEditTeamModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
