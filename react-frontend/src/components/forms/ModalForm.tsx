import { useRef } from "react";

interface ModalFormProps<T> {
  id: string;
  title: string;
  onSubmit: (values: T) => Promise<void> | void;
  onClose?: () => void;
  children: React.ReactNode;
}

export const ModalForm = <T,>({
  id,
  title,
  onSubmit,
  onClose,
  children,
}: ModalFormProps<T>) => {
  const formRef = useRef<HTMLFormElement>(null);
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!formRef.current) return;

    const formData = new FormData(formRef.current);
    const values = Object.fromEntries(formData.entries()) as unknown as T;

    await onSubmit(values);

    // Reset the form
    formRef.current.reset();

    // Close the modal
    const dialog = document.getElementById(id) as HTMLDialogElement | null;
    dialog?.close();
  };

  const handleCancel = () => {
    // Reset the form
    formRef.current?.reset();

    // Close the modal
    const dialog = document.getElementById(id) as HTMLDialogElement | null;
    dialog?.close();

    // Call optional onClose callback
    onClose?.();
  };

  return (
    <dialog id={id} className="modal">
      <div className="modal-box">
        <form method="dialog" className="absolute right-2 top-2">
          <button
            className="btn btn-sm btn-circle btn-ghost"
            aria-label="Close"
          >
            ✕
          </button>
        </form>

        <h3 className="font-bold text-lg">{title}</h3>

        <form ref={formRef} onSubmit={handleSubmit} className="space-y-4">
          {children}

          <div className="modal-action">
            <button
              type="button"
              className="btn"
              onClick={() => handleCancel()}
            >
              Cancel
            </button>
            <button type="submit" className="btn btn-primary">
              Submit
            </button>
          </div>
        </form>
      </div>
    </dialog>
  );
};
