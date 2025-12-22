import { useRef, useState } from "react";
import { CancelButton, CloseButton, SubmitButton } from "../Buttons";
import { ErrorAlert } from "../Alert";

interface ModalFormProps<T> {
  id: string;
  title: string;
  onSubmit: (values: T) => Promise<void> | void;
  onClose?: () => void;
  onReset?: () => void;
  children: React.ReactNode;
}

export const ModalForm = <T,>({
  id,
  title,
  onSubmit,
  onClose,
  onReset,
  children,
}: ModalFormProps<T>) => {
  const formRef = useRef<HTMLFormElement>(null);
  const [errors, setErrors] = useState<{ id: string; message: string }[]>([]);

  const addError = (message: string) => {
    setErrors((prev) => [...prev, { id: crypto.randomUUID(), message }]);
  };

  const removeError = (id: string) => {
    setErrors((prev) => prev.filter((error) => error.id !== id));
  };
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!formRef.current) return;

    const formData = new FormData(formRef.current);
    const values = Object.fromEntries(formData.entries()) as unknown as T;

    try {
      await onSubmit(values);

      // Reset the form
      formRef.current.reset();
      onReset && onReset();
      setErrors([]);

      // Close the modal
      const dialog = document.getElementById(id) as HTMLDialogElement | null;
      dialog?.close();
    } catch (error) {
      // Add error to state

      const errorMessage =
        error instanceof Error ? error.message : String(error);

      addError(errorMessage);
    }
  };

  const handleCancel = () => {
    // Reset the form
    formRef.current?.reset();
    onReset && onReset();

    // Close the modal
    const dialog = document.getElementById(id) as HTMLDialogElement | null;
    setErrors([]);
    dialog?.close();

    // Call optional onClose callback
    onClose?.();
  };

  return (
    <dialog id={id} className="modal">
      <div className="modal-box flex flex-col gap-2">
        <form method="dialog" className="absolute right-2 top-2">
          <CloseButton />
        </form>

        <h3 className="font-bold text-lg">{title}</h3>

        {errors.map((e) => (
          <ErrorAlert
            key={e.id}
            message={e.message}
            onClose={() => removeError(e.id)}
          />
        ))}

        <form ref={formRef} onSubmit={handleSubmit} className="space-y-4">
          {children}

          <div className="modal-action">
            <CancelButton onClick={handleCancel} />
            <SubmitButton />
          </div>
        </form>
      </div>
    </dialog>
  );
};
