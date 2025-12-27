interface Props {
  archived: boolean;
  onClick: React.MouseEventHandler<HTMLButtonElement>;
}

export const ToggleArchivedStatus = ({ archived, onClick }: Props) => {
  return (
    <button type="button" onClick={onClick}>
      {archived ? "restore" : "archive"}
    </button>
  );
};
