import { createContext, useContext, useState } from "react";
import { CloseButton } from "../Buttons";
import { ErrorAlert } from "../Alert";

interface ModalContextValue {
  addError: (message: string) => void;
  clearErrors: () => void;
  closeDialog: () => void;
}

const ModalContext = createContext<ModalContextValue | null>(null);

export function useModal() {
  const context = useContext(ModalContext);
  if (!context) {
    throw new Error("useModal must be used within a Modal");
  }
  return context;
}

interface Props {
  id: string;
  children: React.ReactNode;
  title: string;
  onClose?: () => void;
}

export const Modal = ({ id, children, title, onClose }: Props) => {
  const [errors, setErrors] = useState<{ id: string; message: string }[]>([]);
  const addError = (message: string) => {
    setErrors((prev) => [...prev, { id: crypto.randomUUID(), message }]);
  };
  const removeError = (errorId: string) => {
    setErrors((prev) => prev.filter((error) => error.id !== errorId));
  };
  const closeDialog = () => {
    const el = document.getElementById(id);
    if (el instanceof HTMLDialogElement) el.close();
    setErrors([]);
    onClose?.();
  };

  return (
    <dialog id={id} className="modal">
      <div className="modal-box flex flex-col gap-2 max-h-[90dvh] overflow-y-auto overscroll-contain touch-pan-y">
        <form method="dialog" className="absolute right-2 top-2">
          <CloseButton onClick={closeDialog} />
        </form>
        <h3 className="font-bold text-lg">{title}</h3>

        {errors.map((e) => (
          <ErrorAlert
            key={e.id}
            message={e.message}
            onClose={() => removeError(e.id)}
          />
        ))}
        <ModalContext.Provider value={{ addError, clearErrors: () => setErrors([]), closeDialog }}>
          {children}
        </ModalContext.Provider>
      </div>
    </dialog>
  );
};
