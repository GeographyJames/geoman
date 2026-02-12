import { useState } from "react";
import { useForm } from "react-hook-form";
import { useCreateApiKey } from "@/hooks/api/useCreateApiKey";
import { useQueryClient } from "@tanstack/react-query";
import { Modal, useModal } from "@/components/forms/Modal";
import { CancelButton, SubmitButton } from "@/components/Buttons";
import { ApiError } from "@/lib/api";
import { AlertCircle, Copy } from "lucide-react";

interface CreateKeyFormData {
  key_name: string;
}

const CreateKeyInner = ({
  generatedKey,
  setGeneratedKey,
}: {
  generatedKey: string | null;
  setGeneratedKey: (key: string | null) => void;
}) => {
  const { mutate: createApiKey, isPending } = useCreateApiKey();
  const queryClient = useQueryClient();
  const { addError, closeDialog } = useModal();

  const {
    register,
    handleSubmit,
    reset,
    formState: { isValid },
  } = useForm<CreateKeyFormData>({
    defaultValues: { key_name: "" },
  });

  const onSubmit = (data: CreateKeyFormData) => {
    createApiKey(data, {
      onSuccess: (result) => {
        setGeneratedKey(result.api_key);
        reset();
        queryClient.invalidateQueries({ queryKey: ["apiKeys"] });
      },
      onError: (error) => {
        const message =
          error instanceof ApiError && error.status === 500
            ? "internal server error"
            : error.message;
        addError(`Unable to create API key: ${message}`);
      },
    });
  };

  const handleCancel = () => {
    reset();
    closeDialog();
  };

  const handleDone = () => {
    setGeneratedKey(null);
    closeDialog();
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  if (generatedKey) {
    return (
      <div className="space-y-4">
        <div className="alert alert-warning">
          <AlertCircle size={20} />
          <div className="text-sm">
            <div className="font-semibold">Save this key now!</div>
            <div>
              This is the only time you'll see this key. Make sure to copy it
              somewhere safe.
            </div>
          </div>
        </div>
        <fieldset className="fieldset w-full">
          <legend className="fieldset-legend">Your API Key</legend>
          <div className="join w-full">
            <input
              type="text"
              value={generatedKey}
              readOnly
              className="input input-bordered join-item flex-1 font-mono text-sm"
            />
            <button
              onClick={() => copyToClipboard(generatedKey)}
              className="btn join-item"
              title="Copy to clipboard"
            >
              <Copy size={18} />
            </button>
          </div>
        </fieldset>
        <div className="modal-action">
          <button onClick={handleDone} className="btn btn-primary w-full">
            Done
          </button>
        </div>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)} className="space-y-4">
      <p className="text-sm text-base-content/70">
        Give your API key a descriptive name to help you identify it later.
      </p>
      <fieldset className="fieldset w-full">
        <legend className="fieldset-legend">Key name</legend>
        <input
          type="text"
          {...register("key_name", { required: true })}
          placeholder="e.g., My QGIS Desktop"
          className="input input-bordered w-full"
          autoFocus
        />
      </fieldset>
      <div className="modal-action">
        <CancelButton onClick={handleCancel} disabled={isPending} />
        <SubmitButton
          text="Create key"
          loadingText="Creating..."
          loading={isPending}
          disabled={!isValid}
        />
      </div>
    </form>
  );
};

export const CreateKeyForm = () => {
  const [generatedKey, setGeneratedKey] = useState<string | null>(null);

  return (
    <Modal
      id="create_key"
      title="Create new API key"
      onClose={() => setGeneratedKey(null)}
    >
      <CreateKeyInner
        generatedKey={generatedKey}
        setGeneratedKey={setGeneratedKey}
      />
    </Modal>
  );
};
