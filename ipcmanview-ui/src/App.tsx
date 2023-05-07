import { Route, Routes, useNavigate } from "@solidjs/router";
import { Component, createEffect, on, Show } from "solid-js";
import Home from "./pages/Home";
import Login from "./pages/Login";
import NavBar from "./components/NavBar";
import Header from "./components/Header";
import { authStore } from "./pb";

const App: Component = () => {
  const navigate = useNavigate();

  // Navigate to / on logout
  createEffect(
    on(authStore, (now, old) => {
      if (old?.isValid && !now.isValid) {
        navigate("/", { replace: true });
      }
    })
  );

  return (
    <Show when={authStore().isValid} fallback={<Login />}>
      <div class="flex h-screen w-screen flex-col">
        <div>
          <Header class="h-14" />
        </div>
        <div class="flex h-full flex-col overflow-hidden sm:flex-row">
          <div>
            <NavBar class="h-14 w-full flex-row sm:h-full sm:w-14 sm:flex-col" />
          </div>
          <div class="w-full overflow-auto p-2">
            <Routes>
              <Route path="/" component={Home} />
            </Routes>
          </div>
        </div>
      </div>
    </Show>
  );
};

export default App;
