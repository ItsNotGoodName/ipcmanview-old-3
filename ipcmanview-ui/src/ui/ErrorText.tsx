import { styled } from "@macaron-css/solid";

import { theme } from "./theme";

const ErrorText = styled("div", {
  base: {
    color: theme.color.Red,
  },
});

export default ErrorText;
