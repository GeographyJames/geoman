import type { ReactNode } from "react";

interface SelectProps {
  name: string;
  label: string;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  value?: string;
  children: ReactNode;
  onChange?: (value: string) => void;
  defaultValue?: string;
}

export const Select = ({
  name,
  label,
  required,
  value,
  onChange,
  children,
  defaultValue,
}: SelectProps) => {
  return (
    <fieldset className="fieldset">
      <legend className="fieldset-legend">{label}</legend>

      <select
        defaultValue={defaultValue}
        required={required}
        name={name}
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
        className="select select-bordered w-full"
      >
        {children}
      </select>
    </fieldset>
  );
};
