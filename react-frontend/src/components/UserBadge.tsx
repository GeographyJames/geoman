import { SignedIn, UserButton } from "@clerk/clerk-react";
import { useEffect, useState } from "react";

function useIsSmallScreen() {
  const [isSmall, setIsSmall] = useState(false);

  useEffect(() => {
    const mq = window.matchMedia("(max-width: 640px)");
    const update = () => setIsSmall(mq.matches);
    update();
    mq.addEventListener("change", update);
    return () => mq.removeEventListener("change", update);
  }, []);

  return isSmall;
}

export const UserBadge = () => {
  const isSmallScreen = useIsSmallScreen();

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
          <UserButton
            appearance={
              isSmallScreen
                ? {
                    elements: {
                      userButtonPopoverCard: `
                        !fixed
                        !top-1/2
                        !left-1/2
                        !-translate-x-1/2
                        !-translate-y-1/2
                        !max-w-[90vw]
                      `,
                    },
                  }
                : undefined
            }
          />
        </SignedIn>
      )}
    </>
  );
};
