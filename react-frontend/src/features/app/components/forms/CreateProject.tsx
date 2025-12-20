import type ProjectInputDTO from "@/domain/project/inputDTO";
import { TextInput } from "../../../../components/forms/components/TextInput";
import { ModalForm } from "../../../../components/forms/ModalForm";
import { usePostProject } from "../../hooks/usePostProject";

export const CreateProjectForm = () => {
  const postProject = usePostProject();

  const handleSubmit = (values: { projectName: string }) => {
    let dto: ProjectInputDTO = { name: values.projectName, country_code: "GB" };
    postProject.mutate(dto);
  };
  return (
    <ModalForm
      id="create_project"
      title="Create project"
      onSubmit={handleSubmit}
    >
      <TextInput name="projectName" label="Project name" required />
    </ModalForm>
  );
};
