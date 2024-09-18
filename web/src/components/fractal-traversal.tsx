import { FRACTAL } from "@/api";
import { useFractalVisualization } from "@/hooks";
import { useQuery } from "@apollo/client";
import { useEffect } from "react";

export const FractalTraversal: React.FC = () => {
  const { data, loading, error } = useQuery(FRACTAL);
  const { traverseAndVisualizeGraph, visualization } =
    useFractalVisualization();

  useEffect(() => {
    if (data && data.fractal) {
      traverseAndVisualizeGraph(data.fractal);
    }
  }, [data]);

  return (
    <>
      {loading && <p>Loading...</p>}
      <pre
        className="
        text-white
        rounded-md
        overflow-x-auto
        max-w-full
      "
      >
        {visualization}
      </pre>
      {error && <p>Error: {error.message}</p>}
    </>
  );
};
