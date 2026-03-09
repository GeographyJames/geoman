import { useForm } from "react-hook-form";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostTeam } from "@/hooks/api/usePostTeam";
import { useBusinessUnits } from "@/hooks/api/useBusinessUnits";
import { ApiError } from "@/lib/api";

interface CreateTeamFormData {
  name: string;
  businessUnitId: number | "";
}

const MODAL_ID = "create_team";

const CreateTeamInner = () => {
  const { mutate: postTeam, isPending } = usePostTeam();
  const { data: businessUnits = [] } = useBusinessUnits();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    reset,
    formState: { errors },
  } = useForm<CreateTeamFormData>({
    defaultValues: { name: "", businessUnitId: undefined },
  });

  const onSubmit = (data: CreateTeamFormData) => {
    postTeam(
      { name: data.name, business_unit: data.businessUnitId ? Number(data.businessUnitId) : null },
      {
        onSuccess: () => {
          reset();
          closeDialog();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to create team: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    reset();
    closeDialog();
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Team name</legend>
        <input
          type="text"
          placeholder="Enter team name"
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
        <SubmitButton text="Create team" loadingText="Creating..." loading={isPending} />
      </div>
    </form>
  );
};

export const CreateTeamForm = () => (
  <Modal id={MODAL_ID} title="Create team">
    <CreateTeamInner />
  </Modal>
);

export const openCreateTeamModal = () => {
  const el = document.getElementById(MODAL_ID);
  if (el instanceof HTMLDialogElement) el.showModal();
};
