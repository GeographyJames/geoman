import type ProjectInputDTO from "@/domain/project/inputDTO";
import { ModalForm } from "../../../../components/forms/ModalForm";
import { usePostProject } from "../../../../hooks/api/projects/usePostProject";
import { slugify } from "@/lib/slugify";
import { Visibility } from "@/domain/types";
import { useForm } from "react-hook-form";
import { useAppSettings } from "@/hooks/api/useAppSettings";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";
import { useEffect } from "react";
import { ProjectForm, type ProjectFormData } from "./ProjectForm";

export const CreateProjectForm = () => {
  const postProject = usePostProject();
  const { data: appSettings, isLoading: isLoadingSettings } = useAppSettings();
  const { data: currentUser, isLoading: isLoadingUser } = useCurrentUser();
  const isLoading = isLoadingSettings || isLoadingUser;
  const defautlSrid = currentUser?.operatingCountryId === "GB" ? 27700 : "";

  const { handleSubmit, watch, reset, setValue, control } =
    useForm<ProjectFormData>({
      defaultValues: {
        projectName: "",
        country: currentUser?.operatingCountryId || "",
        srid: defautlSrid,
        visibility: Visibility.Private,
        technologies: [],
      },
    });

  const projectName = watch("projectName");
  const slug = slugify(projectName);

  // Update form defaults when user data loads
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

  const onSubmit = async (data: ProjectFormData) => {
    const dto: ProjectInputDTO = {
      name: data.projectName,
      slug: slug,
      country_code: data.country,
      technologies: data.technologies,
      visibility: data.visibility,
      crs_srid: data.srid !== "" ? data.srid : undefined,
    };

    await postProject.mutateAsync(dto);
  };

  const onReset = () => {
    reset();
  };

  if (isLoading) {
    return (
      <ModalForm
        id="create_project"
        title="Create project"
        onSubmit={handleSubmit(onSubmit)}
        onReset={onReset}
      >
        <div className="flex justify-center items-center py-8">
          <span className="loading loading-spinner loading-lg"></span>
        </div>
      </ModalForm>
    );
  }

  if (!currentUser || !appSettings) {
    return null;
  }

  return (
    <ModalForm
      id="create_project"
      title="Create project"
      onSubmit={handleSubmit(onSubmit)}
      onReset={onReset}
    >
      <ProjectForm
        control={control}
        watch={watch}
        setValue={setValue}
        technologies={appSettings.technologies}
      />
    </ModalForm>
  );
};
