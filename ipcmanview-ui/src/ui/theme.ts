import { createTheme } from "@macaron-css/core";

const space = {
  "0": "0px",
  px: "1px",
  "0.5": "0.125rem",
  "1": "0.25rem",
  "1.5": "0.375rem",
  "2": "0.5rem",
  "2.5": "0.625rem",
  "3": "0.75rem",
  "3.5": "0.875rem",
  "4": "1rem",
  "5": "1.25rem",
  "6": "1.5rem",
  "7": "1.75rem",
  "8": "2rem",
  "9": "2.25rem",
  "10": "2.5rem",
  "11": "2.75rem",
  "12": "3rem",
  "14": "3.5rem",
  "16": "4rem",
  "20": "5rem",
  "24": "6rem",
  "28": "7rem",
  "32": "8rem",
  "36": "9rem",
  "40": "10rem",
  "44": "11rem",
  "48": "12rem",
  "52": "13rem",
  "56": "14rem",
  "60": "15rem",
  "64": "16rem",
  "72": "18rem",
  "80": "20rem",
  "96": "24rem",
};

const latte = {
  Rosewater: "#dc8a78",
  Flamingo: "#dd7878",
  Pink: "#ea76cb",
  Mauve: "#8839ef",
  Red: "#d20f39",
  Maroon: "#e64553",
  Peach: "#fe640b",
  Yellow: "#df8e1d",
  Green: "#40a02b",
  Teal: "#179299",
  Sky: "#04a5e5",
  Sapphire: "#209fb5",
  Blue: "#1e66f5",
  Lavender: "#7287fd",
  Text: "#4c4f69",
  Subtext1: "#5c5f77",
  Subtext0: "#6c6f85",
  Overlay2: "#7c7f93",
  Overlay1: "#8c8fa1",
  Overlay0: "#9ca0b0",
  Surface2: "#acb0be",
  Surface1: "#bcc0cc",
  Surface0: "#ccd0da",
  Base: "#eff1f5",
  Mantle: "#e6e9ef",
  Crust: "#dce0e8",
};

const mocha = {
  Rosewater: "#f5e0dc",
  Flamingo: "#f2cdcd",
  Pink: "#f5c2e7",
  Mauve: "#cba6f7",
  Red: "#f38ba8",
  Maroon: "#eba0ac",
  Peach: "#fab387",
  Yellow: "#f9e2af",
  Green: "#a6e3a1",
  Teal: "#94e2d5",
  Sky: "#89dceb",
  Sapphire: "#74c7ec",
  Blue: "#89b4fa",
  Lavender: "#b4befe",
  Text: "#cdd6f4",
  Subtext1: "#bac2de",
  Subtext0: "#a6adc8",
  Overlay2: "#9399b2",
  Overlay1: "#7f849c",
  Overlay0: "#6c7086",
  Surface2: "#585b70",
  Surface1: "#45475a",
  Surface0: "#313244",
  Base: "#1e1e2e",
  Mantle: "#181825",
  Crust: "#11111b",
};

const size = {
  sm: "640px",
  md: "768px",
  lg: "1024px",
  xl: "1280px",
  "2xl": "1536px",
};

export const minScreen = {
  sm: "screen and (min-width: 640px)",
  md: "screen and (min-width: 768px)",
  lg: "screen and (min-width: 1024px)",
  xl: "screen and (min-width: 1280px)",
  "2xl": "screen and (min-width: 1536px)",
};

export const maxScreen = {
  sm: "screen and (max-width: 639px)",
  md: "screen and (max-width: 767px)",
  lg: "screen and (max-width: 1023px)",
  xl: "screen and (max-width: 1279px)",
  "2xl": "screen and (max-width: 1535px)",
};

const themeDefault = {
  space,
  size,
  borderRadius: "4px",
  opacity: {
    active: "80%",
    disabled: "25%",
  },
};

export const [darkClass, theme] = createTheme({
  ...themeDefault,
  color: {
    ...mocha,
  },
});

export const lightClass = createTheme(theme, {
  ...themeDefault,
  color: {
    ...latte,
  },
});
