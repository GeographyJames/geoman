import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePatchTeam } from "@/hooks/api/usePatchTeam";
import { useBusinessUnits } from "@/hooks/api/useBusinessUnits";
import { ApiError } from "@/lib/api";
import type Team from "@/domain/team/entity";

interface EditTeamFormData {
  name: string;
  businessUnitId: number;
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
      ? { name: team.name, businessUnitId: team.businessUnitId ?? 0 }
      : undefined,
  });

  const onSubmit = (data: EditTeamFormData) => {
    if (!team) return;
    patchTeam(
      {
        teamId: team.id,
        patch: {
          name: dirtyFields.name ? data.name : undefined,
          business_unit: dirtyFields.businessUnitId ? Number(data.businessUnitId) : undefined,
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
      <div className="form-control">
        <label className="label" htmlFor="edit-team-name">
          <span className="label-text">Team name</span>
        </label>
        <input
          id="edit-team-name"
          type="text"
          className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Team name is required" })}
        />
        {errors.name && (
          <span className="label-text-alt text-error mt-1">{errors.name.message}</span>
        )}
      </div>

      <div className="form-control">
        <label className="label" htmlFor="edit-team-bu">
          <span className="label-text">Business unit</span>
        </label>
        <select
          id="edit-team-bu"
          className={`select select-bordered w-full ${errors.businessUnitId ? "select-error" : ""}`}
          {...register("businessUnitId", { required: "Business unit is required" })}
        >
          <option value="">Select a business unit…</option>
          {businessUnits.map((bu) => (
            <option key={bu.id} value={bu.id}>
              {bu.name}
            </option>
          ))}
        </select>
        {errors.businessUnitId && (
          <span className="label-text-alt text-error mt-1">{errors.businessUnitId.message}</span>
        )}
      </div>

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
