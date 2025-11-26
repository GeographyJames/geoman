import LandingPage from "@/components/LandingPage";
import { SignedIn, SignedOut, SignIn, UserButton } from "@clerk/clerk-react";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
  component: App,
});

export default function App() {
  return <LandingPage />;
}
