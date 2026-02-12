import type ProjectInputDTO from "@/domain/project/inputDTO";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { usePostProject } from "../../../../hooks/api/projects/usePostProject";
import { slugify } from "@/lib/slugify";
import { Visibility } from "@/domain/types";
import { useForm } from "react-hook-form";
import { useAppSettings } from "@/hooks/api/useAppSettings";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";
import { useEffect } from "react";
import { ProjectForm, type ProjectFormData } from "./ProjectForm";
import { ApiError } from "@/lib/api";

const CreateProjectInner = () => {
  const { mutate: postProject, isPending } = usePostProject();
  const { data: appSettings, isLoading: isLoadingSettings } = useAppSettings();
  const { data: currentUser, isLoading: isLoadingUser } = useCurrentUser();
  const isLoading = isLoadingSettings || isLoadingUser;
  const { addError, closeDialog } = useModal();

  const { handleSubmit, watch, reset, setValue, control } =
    useForm<ProjectFormData>({
      defaultValues: {
        projectName: "",
        country: currentUser?.operatingCountryId || "",
        srid: currentUser?.operatingCountryId === "GB" ? 27700 : "",
        visibility: Visibility.Private,
        technologies: [],
      },
    });

  const projectName = watch("projectName");
  const slug = slugify(projectName);

  useEffect(() => {
    if (currentUser) {
      const defaultSrid = currentUser.operatingCountryId === "GB" ? 27700 : "";
      reset({
        projectName: "",
        country: currentUser.operatingCountryId,
        srid: defaultSrid,
        visibility: Visibility.Private,
        technologies: [],
      });
    }
  }, [currentUser, reset]);

  const onSubmit = (data: ProjectFormData) => {
    const dto: ProjectInputDTO = {
      name: data.projectName,
      slug: slug,
      country_code: data.country,
      technologies: data.technologies,
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

  if (isLoading) {
    return (
      <div className="flex justify-center items-center py-8">
        <span className="loading loading-spinner loading-lg"></span>
      </div>
    );
  }

  if (!currentUser || !appSettings) {
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
