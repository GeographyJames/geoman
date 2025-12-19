interface TextInputProps {
  name: string;
  label: string;
  placeholder?: string;
  required?: boolean;
}

export const TextInput = ({
  name,
  label,
  placeholder,
  required,
}: TextInputProps) => {
  return (
    <label className="form-control w-full">
      <span className="label-text">{label}</span>
      <input
        name={name}
        className="input input-bordered w-full"
        placeholder={placeholder ?? label}
        required={required}
      />
    </label>
  );
};
