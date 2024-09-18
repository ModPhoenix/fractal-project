import { FractalTraversal } from "@/components/fractal-traversal";
import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/traversal")({
  component: () => <FractalTraversal />,
});
