import { CloseButton } from "./Buttons";

type FlashType = "success" | "error" | "warning" | "info";

const alertClass: Record<FlashType, string> = {
  success: "alert-success",
  error: "alert-error",
  warning: "alert-warning",
  info: "alert-info",
};

const closeButtonStyle: Record<FlashType, string> = {
  success: "btn btn-circle btn-success btn-xs border-none",
  error: "btn btn-circle btn-error btn-xs border-none",
  warning: "btn btn-circle btn-warning btn-xs border-none",
  info: "btn btn-circle btn-info btn-xs border-none",
};

export const FlashAlert = ({
  message,
  type,
  onClose,
}: {
  message: string;
  type: FlashType;
  onClose: () => void;
}) => {
  return (
    <div
      role="alert"
      className={`alert ${alertClass[type]} flex justify-between`}
    >
      <span>{message}</span>
      <CloseButton onClick={onClose} style={closeButtonStyle[type]} />
    </div>
  );
};
