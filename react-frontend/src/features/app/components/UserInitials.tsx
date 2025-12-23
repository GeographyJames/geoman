interface Props {
  firstName: string;
  lastName: string;
  tooltip_position?: string;
  message?: string;
}

function UserInitials({
  message,
  firstName,
  lastName,
  tooltip_position,
}: Props) {
  return message ? (
    <div
      className={`tooltip ${
        tooltip_position ? tooltip_position : "tooltip-left"
      }`}
      data-tip={message}
    >
      <Initials firstName={firstName} lastName={lastName} />
    </div>
  ) : (
    <Initials firstName={firstName} lastName={lastName} />
  );
}

export default UserInitials;

const Initials = ({
  firstName,
  lastName,
}: {
  firstName: string;
  lastName: string;
}) => {
  return (
    <div className="flex items-center justify-center w-6 h-6 border-2 border-gray-500 rounded-full content-center">
      <span className="text-xs">
        {(firstName.length > 0 ? firstName[0] : "") +
          (lastName.length > 0 ? lastName[0] : "")}
      </span>
    </div>
  );
};
