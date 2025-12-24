interface Props {
  archived: boolean;
  setArchived: (archived: boolean) => void;
}

export const ToggleArchivedStatus = ({ archived, setArchived }: Props) => {
  return (
    <div onClick={() => setArchived(!archived)}>
      {archived ? "restore" : "archive"}
    </div>
  );
};
