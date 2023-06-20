import { styled } from "@macaron-css/solid";

import { theme } from "./theme";

export const buttonVariants = {
  size: {
    small: {
      padding: `${theme.space["0.5"]} ${theme.space[2]}`,
    },
    medium: {
      padding: `${theme.space[2]} ${theme.space[2]}`,
    },
    large: {
      padding: `${theme.space[4]} ${theme.space[4]}`,
    },
  },
  color: {
    primary: {
      background: theme.color.Mauve,
      color: theme.color.Crust,
    },
    secondary: {
      background: theme.color.Subtext0,
      color: theme.color.Crust,
    },
    success: {
      background: theme.color.Green,
      color: theme.color.Crust,
    },
    danger: {
      background: theme.color.Red,
      color: theme.color.Crust,
    },
  },
};

export const Button = styled("button", {
  base: {
    whiteSpace: "nowrap",
    overflow: "hidden",
    appearance: "none",
    border: "none",
    borderRadius: theme.borderRadius,
    cursor: "pointer",
    ":disabled": {
      cursor: "not-allowed",
      opacity: theme.opacity.disabled,
    },
    selectors: {
      ["&:hover:enabled"]: {
        opacity: theme.opacity.active,
      },
    },
  },
  variants: buttonVariants,
  defaultVariants: {
    size: "medium",
    color: "primary",
  },
});
