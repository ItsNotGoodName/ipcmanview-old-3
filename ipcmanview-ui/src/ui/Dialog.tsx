import { keyframes } from "@macaron-css/core";
import { styled } from "@macaron-css/solid";

import { theme } from "./theme";

const appearAnimation = keyframes({
  from: { transform: "translateY(-20%)", opacity: 0 },
  to: { transform: "translateY(0%)", opacity: 1 },
});

export const Dialog = styled("dialog", {
  base: {
    marginTop: "10vh",
    padding: "0",
    width: "auto",
    maxWidth: theme.size.sm,
    maxHeight: "80vh",
    overflowY: "auto",
    background: "none",
    border: "none",
    color: theme.color.Text,
    selectors: {
      ["&[open]"]: {
        animation: `${appearAnimation} 0.1s`,
      },
    },
  },
});
