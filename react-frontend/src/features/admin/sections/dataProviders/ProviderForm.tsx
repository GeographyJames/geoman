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
  const optional = mode === "create"
    ? <span className="label-text-alt text-base-content/50">optional</span>
    : <span className="label-text-alt text-base-content/50">clear to remove</span>;

  return (
    <>
      <div className="form-control">
        <label className="label" htmlFor="provider-name">
          <span className="label-text">Name</span>
        </label>
        <input
          id="provider-name"
          type="text"
          placeholder={mode === "create" ? "e.g. Natural England" : undefined}
          className={`input input-bordered w-full ${errors.name ? "input-error" : ""}`}
          {...register("name", { required: "Name is required" })}
        />
        {errors.name && <span className="label-text-alt text-error mt-1">{errors.name.message}</span>}
      </div>

      <div className="grid grid-cols-2 gap-3">
        <div className="form-control">
          <label className="label" htmlFor="provider-country">
            <span className="label-text">Country code</span>
            {optional}
          </label>
          <input
            id="provider-country"
            type="text"
            placeholder="e.g. GB"
            className="input input-bordered w-full"
            {...register("country_code")}
          />
        </div>
        <div className="form-control">
          <label className="label" htmlFor="provider-subdivision">
            <span className="label-text">Subdivision</span>
            {optional}
          </label>
          <input
            id="provider-subdivision"
            type="text"
            placeholder="e.g. GB-ENG"
            className="input input-bordered w-full"
            {...register("subdivision")}
          />
        </div>
      </div>
    </>
  );
};
