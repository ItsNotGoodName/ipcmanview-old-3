import { styled } from "@macaron-css/solid";

import { theme } from "./theme";

export const Card = styled("div", {
  base: {
    border: `${theme.space.px} solid ${theme.color.Overlay0}`,
    borderRadius: theme.borderRadius,
    background: theme.color.Surface0,
  },
});

export const CardHeader = styled("div", {
  base: {
    overflowX: "auto",
    padding: theme.space[2],
    borderBottom: `${theme.space.px} solid ${theme.color.Overlay0}`,
    background: theme.color.Surface1,
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
  },
});

export const CardBody = styled("div", {
  base: {
    overflowX: "auto",
    padding: theme.space[4],
  },
});
