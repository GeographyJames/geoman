import type { ApiKey } from "@/hooks/api/useApiKeys";
import { useRevokeApiKey } from "@/hooks/api/useRevokeApiKey";
import { useQueryClient } from "@tanstack/react-query";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { ApiError } from "@/lib/api";

const RevokeKeyInner = ({
  apiKey,
  onClose,
}: {
  apiKey: ApiKey | null;
  onClose: () => void;
}) => {
  const { mutate: revokeApiKey, isPending } = useRevokeApiKey();
  const queryClient = useQueryClient();
  const { addError, closeDialog } = useModal();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!apiKey) return;
    revokeApiKey(apiKey.id, {
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ["apiKeys"] });
        closeDialog();
        onClose();
      },
      onError: (error) => {
        const message =
          error instanceof ApiError && error.status === 500
            ? "internal server error"
            : error.message;
        addError(`Unable to revoke API key: ${message}`);
      },
    });
  };

  const handleCancel = () => {
    closeDialog();
    onClose();
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {apiKey && (
        <p>
          Are you sure you want to revoke{" "}
          <span className="font-bold">{apiKey.name}</span>?
        </p>
      )}
      <p>
        <span className="font-bold">This action cannot be undone.</span>
      </p>
      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton
          text="Revoke"
          colour="btn-error"
          loadingText="Revoking..."
          loading={isPending}
        />
      </div>
    </form>
  );
};

export const RevokeKeyForm = ({
  apiKey,
  onClose,
}: {
  apiKey: ApiKey | null;
  onClose: () => void;
}) => {
  return (
    <Modal id="revoke_key" title="Revoke API key" onClose={onClose}>
      <RevokeKeyInner apiKey={apiKey} onClose={onClose} />
    </Modal>
  );
};
