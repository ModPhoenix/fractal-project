import { FRACTAL, FractalGraphQl } from "@/api";
import { useLazyQuery } from "@apollo/client";
import { DeepPartial } from "@apollo/client/utilities";
import { useState } from "react";

export const useFractalVisualization = () => {
  const [visualization, setVisualization] = useState<string>("");
  const [getFractal] = useLazyQuery(FRACTAL);

  const traverseAndVisualizeGraph = async (
    rootFractal: DeepPartial<FractalGraphQl>
  ) => {
    const visited = new Set<string>();
    let visualizationText = "";

    const dfs = async (
      fractal: DeepPartial<FractalGraphQl>,
      depth: number,
      direction: "down" | "up",
      path: string[] = []
    ) => {
      if (
        !fractal.id ||
        visited.has(`${direction}-${fractal.id}-${path.join(",")}`)
      )
        return;
      visited.add(`${direction}-${fractal.id}-${path.join(",")}`);

      const indent = "  ".repeat(depth);
      const prefix = direction === "down" ? "↓ " : "↑ ";
      visualizationText += `${indent}${prefix}${fractal.name} (ID: ${fractal.id})\n`;

      if (direction === "down") {
        if (!fractal.children || fractal.children.length === 0) {
          // Fetch children if not available
          const { data } = await getFractal({
            variables: { name: fractal.name },
          });
          if (data && data.fractal) {
            fractal = data.fractal;
          }
        }
        if (fractal.children && fractal.children.length > 0) {
          for (const child of fractal.children) {
            if (child) {
              await dfs(child, depth + 1, "down", [...path, fractal.id]);
            }
          }
        }
      }

      if (direction === "up") {
        if (!fractal.parents || fractal.parents.length === 0) {
          // Fetch parents if not available
          const { data } = await getFractal({
            variables: { name: fractal.name },
          });
          if (data && data.fractal) {
            fractal = data.fractal;
          }
        }
        if (fractal.parents && fractal.parents.length > 0) {
          for (const parent of fractal.parents) {
            if (parent) {
              await dfs(parent, depth + 1, "up", [...path, fractal.id]);
            }
          }
        }
      }
    };

    // Traverse down from root to leaves
    visualizationText += "Traversal from root to leaves:\n";
    await dfs(rootFractal, 0, "down");

    // Reset visited set for upward traversal
    visited.clear();
    visualizationText += "\nTraversal from leaves to root:\n";

    // Find all leaf nodes
    const leafNodes: DeepPartial<FractalGraphQl>[] = [];
    const findLeafNodes = (fractal: DeepPartial<FractalGraphQl>) => {
      if (!fractal.children || fractal.children.length === 0) {
        leafNodes.push(fractal);
      } else if (fractal.children && fractal.children.length > 0) {
        for (const child of fractal.children) {
          if (child) {
            findLeafNodes(child);
          }
        }
      }
    };
    findLeafNodes(rootFractal);

    // Traverse up from each leaf to all possible roots
    for (const leaf of leafNodes) {
      await dfs(leaf, 0, "up");
    }

    setVisualization(visualizationText);
  };

  return { traverseAndVisualizeGraph, visualization };
};
