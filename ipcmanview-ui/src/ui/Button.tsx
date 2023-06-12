import { styled } from "@macaron-css/solid";

import { theme } from "./theme";

const Button = styled("button", {
  base: {
    appearance: "none",
    border: "none",
    borderRadius: theme.borderRadius,
    cursor: "pointer",
    ":disabled": {
      cursor: "not-allowed",
      opacity: theme.opacity.disabled,
    },
    selectors: {
      ["&:active:enabled"]: {
        opacity: theme.opacity.active,
      },
    },
  },
  variants: {
    size: {
      small: {
        padding: theme.space["0.5"],
      },
      medium: {
        padding: theme.space[2],
      },
      large: {
        padding: theme.space[4],
      },
    },
    color: {
      primary: {
        background: theme.color.Mauve,
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
  },
  defaultVariants: {
    size: "medium",
    color: "primary",
  },
});

export default Button;
