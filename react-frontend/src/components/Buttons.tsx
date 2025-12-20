import { FaPlus } from "react-icons/fa6";
import { IoMenu } from "react-icons/io5";
import { IoClose } from "react-icons/io5";
import { MdChevronRight } from "react-icons/md";

const CIRCLE_BUTTON_STYLE = "btn btn-ghost btn-circle btn-sm";

export const CancelButton = ({ onClick }: { onClick: () => void }) => {
  return (
    <button type="button" className="btn" onClick={onClick}>
      Cancel
    </button>
  );
};

export const CloseButton = ({ onClick }: { onClick?: () => void }) => {
  return (
    <button
      className={CIRCLE_BUTTON_STYLE}
      aria-label="Close"
      onClick={onClick}
    >
      <IoClose size={18} />
    </button>
  );
};

export const SubmitButton = () => {
  return (
    <button type="submit" className="btn btn-primary">
      Submit
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
    <button className={"btn btn-primary btn-sm"} onClick={onClick}>
      <FaPlus />
      {`Create ${text}`}
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
        className={`transition-transform ${expanded ? "rotate-90" : ""}`}
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
