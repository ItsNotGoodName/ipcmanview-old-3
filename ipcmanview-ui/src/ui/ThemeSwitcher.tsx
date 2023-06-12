import { styled } from "@macaron-css/solid";
import {
  RiDesignContrastFill,
  RiWeatherMoonFill,
  RiWeatherSunFill,
} from "solid-icons/ri";
import { Component, Match, Switch } from "solid-js";

import {
  DARK_MODE,
  LIGHT_MODE,
  themeMode,
  toggleThemeMode,
} from "./theme-mode";
import { theme } from "./theme";

const Root = styled("button", {
  base: {
    padding: 0,
    border: "none",
    background: "none",
    color: theme.color.Text,
    cursor: "pointer",
  },
});

const ThemeSwitcher: Component<{ class?: string; iconClass?: string }> = (
  props
) => (
  <Root onClick={toggleThemeMode} class={props.class}>
    <Switch fallback={<RiDesignContrastFill class={props.iconClass} />}>
      <Match when={themeMode() == DARK_MODE}>
        <RiWeatherMoonFill class={props.iconClass} />
      </Match>
      <Match when={themeMode() == LIGHT_MODE}>
        <RiWeatherSunFill class={props.iconClass} />
      </Match>
    </Switch>
  </Root>
);

export default ThemeSwitcher;
