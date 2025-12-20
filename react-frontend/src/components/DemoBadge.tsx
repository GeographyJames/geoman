import { SignedIn, UserButton } from "@clerk/clerk-react";

export const UserBadge = () => {
  return (
    <>
      {__RUN_ENVIRONMENT__ === "demo" ? (
        <div
          className="tooltip tooltip-left"
          data-tip="User authentication disabled in demo mode"
        >
          <div className="badge badge-warning">Demo</div>
        </div>
      ) : (
        <SignedIn>
          <UserButton />
        </SignedIn>
      )}
    </>
  );
};
