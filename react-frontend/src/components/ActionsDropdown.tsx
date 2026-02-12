import { type ReactNode } from "react";
import { IoMdArrowDropdown } from "react-icons/io";

interface Props {
  id: string | number;
  children: ReactNode;
  style?: string;
}

export const ActionsDropdown = ({ id, children, style }: Props) => {
  const popoverId = `actions-popover-${id}`;
  const anchorName = `--actions-anchor-${id}`;
  return (
    <>
      <button
        className={`btn btn-xs ${style}`}
        popoverTarget={popoverId}
        style={{ anchorName } as React.CSSProperties}
      >
        <IoMdArrowDropdown />
      </button>

      <ul
        className="dropdown dropdown-end menu w-52 rounded-box bg-base-100 shadow-sm overflow-visible"
        popover="auto"
        id={popoverId}
        style={{ positionAnchor: anchorName } as React.CSSProperties}
      >
        {children}
      </ul>
    </>
  );
};
