import { createRoot } from "react-dom/client";
import { App } from "./App.tsx";
import "./index.css";
import { TooltipProvider } from "@/components/ui/tooltip";
import { ThemeProvider } from "./components/theme-provider";
import { ApolloProvider } from "@apollo/client";
import { client } from "@/api";
import { Toaster } from "@/components/ui/toaster";

createRoot(document.getElementById("root")!).render(
  <ApolloProvider client={client}>
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <TooltipProvider>
        <App />
        <Toaster />
      </TooltipProvider>
    </ThemeProvider>
  </ApolloProvider>
);
