import { styled } from "@macaron-css/solid";
import { ParentComponent } from "solid-js";

import { theme } from "./theme";

const Center = styled("div", {
  base: {
    display: "flex",
    justifyContent: "center",
    padding: `${theme.space[16]} ${theme.space[4]} 0 ${theme.space[4]}`,
  },
});

const CenterChild = styled("div", {
  base: {
    flex: "1",
    display: "flex",
    flexDirection: "column",
    gap: theme.space[4],
    maxWidth: theme.space[96],
  },
});

export const LayoutCenter: ParentComponent = (props) => (
  <Center>
    <CenterChild>{props.children}</CenterChild>
  </Center>
);
