import { FractalUi } from "@/components/fractal-ui";
import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/")({
  component: () => <FractalUi />,
});
