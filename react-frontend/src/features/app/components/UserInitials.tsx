import { dateFormat } from "@/constants";
import type User from "@/domain/user/entity";

interface Props {
  user: User;
  date: Date;
  tooltip_position?: string;
}

function UserInitials({ user, date, tooltip_position }: Props) {
  return (
    <div
      className={`tooltip ${
        tooltip_position ? tooltip_position : "tooltip-left"
      }`}
      data-tip={`${user.firstName} ${user.lastName} ${dateFormat.format(date)}`}
    >
      <div className="flex items-center justify-center w-6 h-6 border-2 border-gray-500 rounded-full content-center">
        <span className="text-xs">
          {(user.firstName.length > 0 ? user.firstName[0] : "") +
            (user.lastName.length > 0 ? user.lastName[0] : "")}
        </span>
      </div>
    </div>
  );
}

export default UserInitials;
