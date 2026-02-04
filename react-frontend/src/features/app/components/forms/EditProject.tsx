import { ModalForm } from "@/components/forms/ModalForm";
import { slugify } from "@/lib/slugify";
import { useForm } from "react-hook-form";
import { useAppSettings } from "@/hooks/api/useAppSettings";
import { useEffect } from "react";
import { usePatchProject } from "@/hooks/api/projects/usePatchProject";
import { useEditProject } from "../../contexts/EditProjectContext";
import { ProjectForm, type ProjectFormData } from "./ProjectForm";

export const EditProjectForm = () => {
  const { project, clear } = useEditProject();
  const patchProject = usePatchProject();
  const { data: appSettings, isLoading } = useAppSettings();

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

  // Reset form values when the project changes
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

  const onSubmit = async (data: ProjectFormData) => {
    if (!project) return;

    await patchProject.mutateAsync({
      id: project.id,
      dto: {
        name: data.projectName,
        slug: slug,
        country_code: data.country,
        technologies: data.technologies,
        visibility: data.visibility,
        crs_srid: data.srid !== "" ? data.srid : null,
      },
    });
  };

  const onReset = () => {
    reset();
  };

  if (isLoading) {
    return (
      <ModalForm
        id="edit_project"
        title="Edit project"
        onSubmit={handleSubmit(onSubmit)}
        onReset={onReset}
        onClose={clear}
      >
        <div className="flex justify-center items-center py-8">
          <span className="loading loading-spinner loading-lg"></span>
        </div>
      </ModalForm>
    );
  }

  if (!appSettings) {
    return null;
  }

  return (
    <ModalForm
      id="edit_project"
      title="Edit project"
      onSubmit={handleSubmit(onSubmit)}
      onReset={onReset}
      onClose={clear}
      submitDisabled={!isDirty}
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
