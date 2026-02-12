import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { slugify } from "@/lib/slugify";
import { useForm } from "react-hook-form";
import { useAppSettings } from "@/hooks/api/useAppSettings";
import { useEffect } from "react";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { useEditProject } from "../../contexts/EditProjectContext";
import { ProjectForm, type ProjectFormData } from "./ProjectForm";
import { ApiError } from "@/lib/api";

const EditProjectInner = () => {
  const { project, clear } = useEditProject();
  const { mutate: patchProject, isPending } = usePatchProject();
  const { data: appSettings, isLoading } = useAppSettings();
  const { addError, closeDialog } = useModal();

  const {
    handleSubmit,
    watch,
    reset,
    setValue,
    control,
    formState: { isDirty },
  } = useForm<ProjectFormData>({
    defaultValues: {
      projectName: "",
      country: "",
      srid: "",
      visibility: "PRIVATE",
      technologies: [],
    },
  });

  const projectName = watch("projectName");
  const slug = slugify(projectName);

  useEffect(() => {
    if (project && appSettings) {
      const techIds = appSettings.technologies
        .filter((t) =>
          project.outputDto.properties.technologies.includes(t.name),
        )
        .map((t) => t.id);

      reset({
        projectName: project.name,
        country: project.outputDto.properties.country_code,
        srid: project.outputDto.properties.crs_srid ?? "",
        visibility: project.visibility,
        technologies: techIds,
      });
    }
  }, [project, appSettings, reset]);

  const onSubmit = (data: ProjectFormData) => {
    if (!project) return;

    patchProject(
      {
        id: project.id,
        dto: {
          name: data.projectName,
          slug: slug,
          country_code: data.country,
          technologies: data.technologies,
          visibility: data.visibility,
          crs_srid: data.srid !== "" ? data.srid : null,
        },
      },
      {
        onSuccess: () => {
          reset();
          closeDialog();
          clear();
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "Unable to edit project: internal server error"
              : error.message;
          addError(message);
        },
      },
    );
  };

  const handleCancel = () => {
    reset();
    closeDialog();
    clear();
  };

  if (isLoading) {
    return (
      <div className="flex justify-center items-center py-8">
        <span className="loading loading-spinner loading-lg"></span>
      </div>
    );
  }

  if (!appSettings) {
    return null;
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <ProjectForm
        control={control}
        watch={watch}
        setValue={setValue}
        technologies={appSettings.technologies}
      />
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

export const EditProjectForm = () => {
  return (
    <Modal id="edit_project" title="Edit project" onClose={useEditProject().clear}>
      <EditProjectInner />
    </Modal>
  );
};
