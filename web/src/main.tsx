import { createRoot } from "react-dom/client";
import "./index.css";
import { TooltipProvider } from "@/components/ui/tooltip";
import { ThemeProvider } from "./components/theme-provider";
import { ApolloProvider } from "@apollo/client";
import { client } from "@/api";
import { Toaster } from "@/components/ui/toaster";
import { RouterProvider, createRouter } from "@tanstack/react-router";
import { routeTree } from "./routeTree.gen";

// Create a new router instance
const router = createRouter({ routeTree });
// Register the router instance for type safety
declare module "@tanstack/react-router" {
  interface Register {
    router: typeof router;
  }
}

createRoot(document.getElementById("root")!).render(
  <ApolloProvider client={client}>
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <TooltipProvider>
        <RouterProvider router={router} />
        <Toaster />
      </TooltipProvider>
    </ThemeProvider>
  </ApolloProvider>
);
