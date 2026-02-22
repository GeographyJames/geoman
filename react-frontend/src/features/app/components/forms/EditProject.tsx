import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { slugify } from "@/lib/slugify";
import { useForm } from "react-hook-form";

import { useEffect } from "react";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { useEditProject } from "../../contexts/EditProjectContext";
import { ProjectForm, type ProjectFormData } from "./ProjectForm";
import { ApiError } from "@/lib/api";
import { useNavigate } from "@tanstack/react-router";

const EditProjectInner = () => {
  const { project, clear } = useEditProject();
  const { mutate: patchProject, isPending } = usePatchProject();
  const navigate = useNavigate();

  const { addError, closeDialog } = useModal();

  const {
    handleSubmit,
    watch,
    reset,
    control,
    formState: { isDirty },
  } = useForm<ProjectFormData>({
    defaultValues: {
      projectName: "",
      srid: "",
      visibility: "PRIVATE",
    },
  });

  const projectName = watch("projectName");
  const slug = slugify(projectName);

  useEffect(() => {
    if (project) {
      reset({
        projectName: project.name,
        srid: project.outputDto.properties.crs_srid ?? "",
        visibility: project.visibility,
      });
    }
  }, [project, reset]);

  const onSubmit = (data: ProjectFormData) => {
    if (!project) return;

    patchProject(
      {
        id: project.id,
        dto: {
          name: data.projectName,
          slug: slug,
          visibility: data.visibility,
          crs_srid: data.srid !== "" ? data.srid : null,
        },
      },
      {
        onSuccess: () => {
          reset();
          closeDialog();
          clear();
          const oldSlug = project.slug;
          navigate({
            from: "/",
            search: (prev) => {
              if (!prev.projects) return prev;
              const slugs = prev.projects.split(",");
              const idx = slugs.indexOf(oldSlug);
              if (idx === -1) return prev;
              slugs[idx] = slug;
              return { ...prev, projects: slugs.join(",") };
            },
            replace: true,
          });
        },
        onError: (error) => {
          const message =
            error instanceof ApiError && error.status === 500
              ? "internal server error"
              : error.message;
          addError(`Unable to update project: ${message}`);
        },
      },
    );
  };

  const handleCancel = () => {
    reset();
    closeDialog();
    clear();
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <ProjectForm control={control} watch={watch} />
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
    <Modal
      id="edit_project"
      title="Edit project"
      onClose={useEditProject().clear}
    >
      <EditProjectInner />
    </Modal>
  );
};
