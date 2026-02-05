import type { ReactNode } from "react";

interface TextInputProps {
  name: string;
  label: string;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  value?: number | string;
  bottomLabel?: string;
  onChange?: (value: string) => void;
  type?: string;
  step?: number;
  children?: ReactNode;
  defaultValue?: string;
  max?: number;
  min?: number;
  autoFocus?: boolean;
}

export const TextInput = ({
  name,
  label,
  placeholder,
  required,
  disabled,
  value,
  bottomLabel,
  children,
  defaultValue,
  type = "text",
  step,
  max,
  min,
  autoFocus,
  onChange,
}: TextInputProps) => {
  return (
    <fieldset className="fieldset w-full">
      <legend className="fieldset-legend">{label}</legend>
      <div className="flex items-center gap-2">
        <input
          type={type}
          name={name}
          className="input input-bordered w-full"
          placeholder={placeholder}
          required={required}
          disabled={disabled}
          value={value ?? ""}
          onChange={(e) => onChange?.(e.target.value)}
          step={step}
          defaultValue={defaultValue}
          max={max}
          min={min}
          autoFocus={autoFocus}
        />
        {children && children}
      </div>

      {bottomLabel && <p className="label">{bottomLabel}</p>}
    </fieldset>
  );
};
