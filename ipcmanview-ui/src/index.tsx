/* @refresh reload */
import { render } from "solid-js/web";
import { Router } from "@solidjs/router";
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";

import "./index.css";
import App from "./App";
import Login from "./pages/Login";
import { PbProvider } from "./pb";

const queryClient = new QueryClient();
const root = document.getElementById("root");

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    "Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got mispelled?"
  );
}

render(
  () => (
    <Router>
      <QueryClientProvider client={queryClient}>
        <PbProvider login={<Login />}>
          <App />
        </PbProvider>
      </QueryClientProvider>
    </Router>
  ),
  root!
);
