import { styled } from "@macaron-css/solid";

import { theme } from "./theme";
import { utility } from "./utility";

const Dialog = styled("dialog", {
  base: {
    ...utility.shadowXl,
    marginTop: "10vh",
    padding: "0",
    width: "auto",
    maxWidth: theme.size.sm,
    maxHeight: "80vh",
    overflowY: "auto",
    background: "none",
    border: "none",
  },
});

export default Dialog;
