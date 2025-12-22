import { CloseButton } from "./Buttons";

export const ErrorAlert = ({
  message,
  onClose,
}: {
  message: string;
  onClose: () => void;
}) => {
  return (
    <div role="alert" className="alert alert-error flex justify-between">
      <span>{message}</span>
      <CloseButton
        onClick={onClose}
        style="btn btn-circle btn-error btn-xs border-none"
      />
    </div>
  );
};
