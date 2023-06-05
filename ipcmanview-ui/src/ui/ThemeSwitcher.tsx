import { RiWeatherMoonFill, RiWeatherSunFill } from "solid-icons/ri";
import { Accessor, Component, createSignal } from "solid-js";

const LIGHT_THEME = "light";
const DARK_THEME = "dark";

const [theme, toggleTheme] = useTheme();

const ThemeSwitcher: Component = () => {
  return (
    <label
      class="swap-rotate swap"
      classList={{ "swap-active": theme() }}
      onClick={toggleTheme}
    >
      <RiWeatherSunFill class="swap-off h-6 w-6" />
      <RiWeatherMoonFill class="swap-on h-6 w-6" />
    </label>
  );
};

function useTheme(): [Accessor<boolean>, () => void] {
  const [theme, setTheme] = createSignal(get() == DARK_THEME);

  return [
    theme,
    () => {
      if (theme()) {
        set(LIGHT_THEME);
        setTheme(false);
      } else {
        set(DARK_THEME);
        setTheme(true);
      }
    },
  ];
}

function get(): string {
  const theme = localStorage.getItem("theme");
  if (theme) {
    set(theme);
  }
  return document
    .querySelector("html[data-theme]")!
    .getAttribute("data-theme")!;
}

function set(theme: string) {
  document.querySelector("html[data-theme]")!.setAttribute("data-theme", theme);
  localStorage.setItem("theme", theme);
}

export default ThemeSwitcher;
