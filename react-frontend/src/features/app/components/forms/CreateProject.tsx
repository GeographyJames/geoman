import type ProjectInputDTO from "@/domain/project/inputDTO";
import { TextInput } from "../../../../components/forms/components/TextInput";
import { ModalForm } from "../../../../components/forms/ModalForm";
import { usePostProject } from "../../hooks/usePostProject";
import { slugify } from "@/lib/slugify";
import { useState } from "react";

export const CreateProjectForm = () => {
  const postProject = usePostProject();

  const [projectName, setProjectName] = useState("");
  const baseDomain = window.location.origin;

  const slug = slugify(projectName);

  const handleSubmit = async () => {
    const dto: ProjectInputDTO = {
      name: projectName,
      slug: slug,
      country_code: "GB",
    };

    await postProject.mutateAsync(dto);
  };

  return (
    <ModalForm
      id="create_project"
      title="Create project"
      onSubmit={handleSubmit}
      onReset={() => setProjectName("")}
    >
      <div className="flex flex-col gap-2">
        <TextInput
          name="projectName"
          label="Project name"
          required
          value={projectName}
          onChange={setProjectName}
        />

        <TextInput
          name="slug"
          label="URL"
          required
          disabled
          value={`${baseDomain}/project/${slug}`}
        />
      </div>
    </ModalForm>
  );
};
