interface Props {
  archived: boolean;
  onClick: React.MouseEventHandler<HTMLButtonElement>;
  disabled?: boolean;
}

export const ToggleArchivedStatus = ({
  archived,
  onClick,
  disabled,
}: Props) => {
  return (
    <button
      type="button"
      onClick={onClick}
      // disabled={disabled}
      className={disabled ? "text-base-content/50" : ""}
    >
      {archived ? "restore" : "archive"}
    </button>
  );
};
