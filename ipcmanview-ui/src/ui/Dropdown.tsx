import { styled } from "@macaron-css/solid";
import { Accessor, Component, createSignal, JSX } from "solid-js";

import { theme } from "./theme";
import { utility } from "./utility";

const Details = styled("details", {
  base: {
    display: "inline-block",
    position: "relative",
  },
});

type DropdownProps = {
  children: (props: {
    open: Accessor<boolean>;
    close: () => void;
  }) => JSX.Element;
};

const Dropdown: Component<DropdownProps> = (props) => {
  const [open, setOpen] = createSignal(false);
  let det: HTMLDetailsElement;
  const close = () => (det.open = false);

  return (
    <Details
      ref={det!}
      onToggle={() => {
        setOpen(det.open);
      }}
      onFocusOut={(e) => {
        if (!e.relatedTarget || !det.contains(e.relatedTarget as Node))
          det.open = false;
      }}
    >
      <props.children open={open} close={close} />
    </Details>
  );
};

export const DropdownButton = styled("summary", {
  base: {
    cursor: "pointer",
    selectors: {
      ["&::marker"]: {
        content: "",
      },
    },
  },
});

export const DropdownContent = styled("div", {
  base: {
    ...utility.shadow,
    zIndex: 10,
    position: "absolute",
    borderRadius: theme.borderRadius,
    minInlineSize: "max-content",
    padding: theme.space[2],
    backgroundColor: theme.color.Surface0,
    border: `${theme.space.px} solid ${theme.color.Overlay0}`,
  },
  variants: {
    position: {
      end: {
        right: 1,
        marginTop: theme.space[1],
      },
    },
  },
});

export default Dropdown;
