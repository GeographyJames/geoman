import type { FieldErrors, UseFormRegister } from "react-hook-form";

export interface ProviderFormData {
  name: string;
  country_code: string;
  subdivision: string;
}

export const PROVIDER_FORM_DEFAULTS: ProviderFormData = {
  name: "",
  country_code: "",
  subdivision: "",
};

interface ProviderFormProps {
  register: UseFormRegister<ProviderFormData>;
  errors: FieldErrors<ProviderFormData>;
  mode: "create" | "edit";
}

export const ProviderForm = ({ register, errors, mode }: ProviderFormProps) => {
  const optionalLabel = mode === "create" ? "optional" : "clear to remove";

  return (
    <>
      <fieldset className="fieldset">
        <legend className="fieldset-legend">Name</legend>
        <input
          type="text"
          placeholder={mode === "create" ? "e.g. Natural England" : undefined}
          className={`input w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Name is required" })}
        />
        {errors.name && <p className="label text-error">{errors.name.message}</p>}
      </fieldset>

      <div className="grid grid-cols-2 gap-3">
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Country code</legend>
          <input
            type="text"
            placeholder="e.g. GB"
            className="input w-full"
            {...register("country_code")}
          />
          <p className="label">{optionalLabel}</p>
        </fieldset>
        <fieldset className="fieldset">
          <legend className="fieldset-legend">Subdivision</legend>
          <input
            type="text"
            placeholder="e.g. GB-ENG"
            className="input w-full"
            {...register("subdivision")}
          />
          <p className="label">{optionalLabel}</p>
        </fieldset>
      </div>
    </>
  );
};
