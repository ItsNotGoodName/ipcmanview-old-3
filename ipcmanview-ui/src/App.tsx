import { Component, createEffect, lazy, on, Show } from "solid-js";
import { Route, Routes, useNavigate } from "@solidjs/router";

import { useAuthRefresh } from "./hooks";
import pb, { authStore } from "./pb";
import NavBar from "./components/NavBar";
import Header from "./components/Header";

const Home = lazy(() => import("./pages/Home"));
const Login = lazy(() => import("./pages/Login"));
const Profile = lazy(() => import("./pages/Profile"));

const App: Component = () => {
  const navigate = useNavigate();
  useAuthRefresh(false);

  // Navigate to / on logout
  createEffect(
    on(authStore, (now, old) => {
      if (old?.isValid && !now.isValid) {
        navigate("/", { replace: true });
      }
    })
  );

  const logout = () => {
    pb.authStore.clear();
  };

  return (
    <Show when={authStore().isValid} fallback={<Login />}>
      <div class="flex h-screen w-screen flex-col">
        <div>
          <Header class="h-14" onLogout={logout} />
        </div>
        <div class="flex h-full flex-col overflow-hidden sm:flex-row">
          <div>
            <NavBar class="h-14 w-full flex-row sm:h-full sm:w-14 sm:flex-col" />
          </div>
          <div class="h-full w-full overflow-auto p-4">
            <Routes>
              <Route path="/" component={Home} />
              <Route path="/profile" component={Profile} />
            </Routes>
          </div>
        </div>
      </div>
    </Show>
  );
};

export default App;
