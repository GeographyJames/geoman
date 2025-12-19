import { TextInput } from "./components/TextInput";
import { ModalForm } from "./ModalForm";

interface Props {
  name: string;
  label: string;
  placeholder?: string;
  required?: boolean;
}

export const CreateProjectForm = () => {
  return (
    <ModalForm id="create_project" title="Create project" onSubmit={() => {}}>
      <TextInput name="projectName" label="Project name" required />
    </ModalForm>
  );
};
