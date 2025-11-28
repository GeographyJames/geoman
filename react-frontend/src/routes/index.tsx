import LandingPage from "@/components/LandingPage";
import { SignedOut, useUser } from "@clerk/clerk-react";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { useEffect } from "react";

export const Route = createFileRoute("/")({
  component: App,
});

export default function App() {
  const { isSignedIn, isLoaded } = useUser();
  const navigate = useNavigate();

  useEffect(() => {
    if (isLoaded && isSignedIn) {
      navigate({ to: "/admin" });
    }
  }, [isLoaded, isSignedIn, navigate]);

  return (
    <SignedOut>
      <LandingPage />
    </SignedOut>
  );
}

