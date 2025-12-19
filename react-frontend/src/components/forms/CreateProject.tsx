import { usePostProject } from "@/hooks/api/project/usePostProject";
import { TextInput } from "./components/TextInput";
import { ModalForm } from "./ModalForm";
import type ProjectInputDto from "@/domain/project/inputDTO";

interface Props {
  name: string;
  label: string;
  placeholder?: string;
  required?: boolean;
}

export const CreateProjectForm = () => {
  const postProject = usePostProject();

  const handleSubmit = (values: { projectName: string }) => {
    let dto: ProjectInputDto = { name: values.projectName, country_code: "GB" };
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
