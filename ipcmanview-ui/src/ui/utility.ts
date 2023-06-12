import { CSSProperties, keyframes } from "@macaron-css/core";
import { styled } from "@macaron-css/solid";

import { theme } from "./theme";

const rotate = keyframes({
  from: { transform: "rotate(0deg)" },
  to: { transform: "rotate(360deg)" },
});

export const utility = {
  animateSpin: {
    animation: `${rotate} 1s linear infinite`,
  } satisfies CSSProperties,

  shadow: {
    boxShadow: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
  } satisfies CSSProperties,

  shadowXl: {
    boxShadow: `0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)`,
  } satisfies CSSProperties,

  icon(space?: keyof (typeof theme)["space"]) {
    space = space ?? "6";
    return {
      height: theme.space[space],
      width: theme.space[space],
    } satisfies CSSProperties;
  },
};

export const Row = styled("div", {
  base: {
    display: "flex",
    alignItems: "center",
  },
  variants: {
    gap: {
      1: {
        gap: theme.space[1],
      },
      2: {
        gap: theme.space[2],
      },
      4: {
        gap: theme.space[4],
      },
    },
  },
});

export const Stack = styled("div", {
  base: {
    display: "flex",
    flexDirection: "column",
  },
  variants: {
    gap: {
      1: {
        gap: theme.space[1],
      },
      2: {
        gap: theme.space[2],
      },
      4: {
        gap: theme.space[4],
      },
    },
  },
});
