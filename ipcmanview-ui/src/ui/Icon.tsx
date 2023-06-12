import { RiSystemAlertFill, RiSystemLoader4Fill } from "solid-icons/ri";

import { utility } from "./utility";
import { theme } from "./theme";
import { styled } from "@macaron-css/solid";

export const IconSpinner = styled(RiSystemLoader4Fill, {
  base: { ...utility.animateSpin },
});

export const IconAlert = styled(RiSystemAlertFill, {
  base: { fill: theme.color.Red },
});
