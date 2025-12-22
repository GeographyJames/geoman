interface TextInputProps {
  name: string;
  label: string;
  placeholder?: string;
  required?: boolean;
  disabled?: boolean;
  value?: string;
  onChange?: (value: string) => void;
}

export const TextInput = ({
  name,
  label,
  placeholder,
  required,
  disabled,
  value,
  onChange,
}: TextInputProps) => {
  return (
    <label className="form-control w-full">
      <span className="label-text">{label}</span>

      <input
        type="text"
        name={name}
        className="input input-bordered w-full"
        placeholder={placeholder ?? label}
        required={required}
        disabled={disabled}
        value={value}
        onChange={(e) => onChange?.(e.target.value)}
      />
    </label>
  );
};
