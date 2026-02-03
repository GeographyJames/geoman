import { FaPlus } from "react-icons/fa6";
import { IoMenu } from "react-icons/io5";
import { IoClose } from "react-icons/io5";
import { MdChevronRight } from "react-icons/md";

const CIRCLE_BUTTON_STYLE = "btn btn-ghost btn-circle btn-sm ";

export const CancelButton = ({ onClick }: { onClick: () => void }) => {
  return (
    <button type="button" className="btn" onClick={onClick}>
      Cancel
    </button>
  );
};

export const CloseButton = ({
  onClick,
  style,
}: {
  onClick?: () => void;
  style?: string;
}) => {
  return (
    <button
      className={style ? style : CIRCLE_BUTTON_STYLE}
      aria-label="Close"
      onClick={onClick}
    >
      <IoClose size={18} />
    </button>
  );
};

export const SubmitButton = ({
  text,
  colour,
}: {
  text?: string;
  colour?: string;
}) => {
  return (
    <button type="submit" className={`btn ${colour ? colour : "btn-primary"}`}>
      {text ? text : "Submit"}
    </button>
  );
};

export const CreateButton = ({
  text,
  onClick,
}: {
  text: string;
  onClick: () => void;
}) => {
  return (
    <button className={"btn btn-primary btn-sm w-32"} onClick={onClick}>
      <FaPlus />
      {text}
    </button>
  );
};

export const ExpandButton = ({
  expanded,
  onClick,
}: {
  expanded: boolean;
  onClick: () => void;
}) => {
  return (
    <button type="button" onClick={onClick} className={CIRCLE_BUTTON_STYLE}>
      <MdChevronRight
        size={24}
        className={`transition-transform ${expanded ? "rotate-270" : "rotate-90"}`}
      />
    </button>
  );
};

export const MenuButton = ({ onClick }: { onClick: () => void }) => {
  return (
    <button type="button" onClick={onClick} className={CIRCLE_BUTTON_STYLE}>
      <IoMenu size={24} />
    </button>
  );
};
