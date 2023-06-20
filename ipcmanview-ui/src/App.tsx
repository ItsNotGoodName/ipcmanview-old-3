import { Component, Show } from "solid-js";
import { useLocation } from "@solidjs/router";
import { styled } from "@macaron-css/solid";
import { globalStyle } from "@macaron-css/core";

import { PbProvider } from "~/data/pb";
import { theme } from "~/ui/theme";
import { themeModeClass } from "~/ui/theme-mode";

import { Application } from "~/views/Application";
import { Debug } from "~/views/Debug";
import { Loading } from "~/views/Loading";
import { Login } from "~/views/Login";

globalStyle("a", {
  textDecoration: "none",
  color: theme.color.Blue,
});

const Root = styled("div", {
  base: {
    background: theme.color.Base,
    color: theme.color.Text,
    position: "fixed",
    inset: 0,
  },
});

export const App: Component = () => {
  const location = useLocation();

  return (
    <Root class={themeModeClass()}>
      <PbProvider login={<Login />} loading={<Loading />}>
        <Show when={!location.query.debug} fallback={<Debug />}>
          <Application />
        </Show>
      </PbProvider>
    </Root>
  );
};
