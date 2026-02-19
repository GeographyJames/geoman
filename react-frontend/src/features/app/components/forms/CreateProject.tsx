import type ProjectInputDTO from "@/domain/project/inputDTO";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostProject } from "../../../../hooks/api/projects/usePostProject";
import { slugify } from "@/lib/slugify";
import { Visibility } from "@/domain/types";
import { useForm } from "react-hook-form";
import { ProjectForm, type ProjectFormData } from "./ProjectForm";
import { ApiError } from "@/lib/api";

const CreateProjectInner = () => {
  const { mutate: postProject, isPending } = usePostProject();

  const { addError, closeDialog } = useModal();

  const { handleSubmit, watch, reset, control } = useForm<ProjectFormData>({
    defaultValues: {
      projectName: "",
      srid: 27700,
      visibility: Visibility.Public,
    },
  });

  const projectName = watch("projectName");
  const slug = slugify(projectName);

  const onSubmit = (data: ProjectFormData) => {
    const dto: ProjectInputDTO = {
      name: data.projectName,
      slug: slug,
      visibility: data.visibility,
      crs_srid: data.srid !== "" ? data.srid : undefined,
    };

    postProject(dto, {
      onSuccess: () => {
        reset();
        closeDialog();
      },
      onError: (error) => {
        const message =
          error instanceof ApiError && error.status === 500
            ? "internal server error"
            : error.message;
        addError(`Unable to create project: ${message}`);
      },
    });
  };

  const handleCancel = () => {
    reset();
    closeDialog();
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <ProjectForm control={control} watch={watch} />
      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton
          text="Create project"
          loadingText="Creating..."
          loading={isPending}
        />
      </div>
    </form>
  );
};

export const CreateProjectForm = () => {
  return (
    <Modal id="create_project" title="Create project">
      <CreateProjectInner />
    </Modal>
  );
};
