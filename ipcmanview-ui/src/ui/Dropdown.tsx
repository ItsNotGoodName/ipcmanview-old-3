import { keyframes } from "@macaron-css/core";
import { styled } from "@macaron-css/solid";
import {
  Accessor,
  Component,
  createEffect,
  createSignal,
  JSX,
  onCleanup,
  ParentComponent,
  Show,
} from "solid-js";

import { buttonVariants } from "./Button";
import { theme } from "./theme";
import { utility } from "./utility";

const TheDropdown = styled("details", {});

export type DropdownProps = {
  children: (props: {
    open: Accessor<boolean>;
    close: () => void;
  }) => JSX.Element;
};

export const Dropdown: Component<DropdownProps> = (props) => {
  const [open, setOpen] = createSignal(false);
  let det: HTMLDetailsElement;
  const close = () => (det.open = false);

  const onClick = (ev: MouseEvent) => {
    if (!det.contains(ev.target as Node)) {
      det.open = false;
    }
  };

  createEffect(() => {
    if (open()) {
      document.addEventListener("click", onClick);
    } else {
      document.removeEventListener("click", onClick);
    }
  });

  onCleanup(() => {
    document.removeEventListener("click", onClick);
  });

  return (
    <TheDropdown
      ref={det!}
      onToggle={() => {
        setOpen(det.open);
      }}
    >
      <props.children open={open} close={close} />
    </TheDropdown>
  );
};

export const DropdownSummary = styled("summary", {
  base: {
    cursor: "pointer",
    overflow: "hidden",
    selectors: {
      ["&::marker"]: {
        content: "",
      },
    },
  },
});

export const DropdownButton = styled("summary", {
  base: {
    whiteSpace: "nowrap",
    overflow: "hidden",
    borderRadius: theme.borderRadius,
    cursor: "pointer",
    userSelect: "none",
    selectors: {
      ["&::marker"]: {
        content: "",
      },
      ["&:hover"]: {
        opacity: theme.opacity.active,
      },
      [`${TheDropdown}[open] &`]: {
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

const appearAnimation = keyframes({
  from: { transform: "scale(95%)", opacity: 0 },
  to: { transform: "scale(100%)", opacity: 1 },
});

const TheDropdownEnd = styled("div", {
  base: {
    display: "flex",
    flexDirection: "row-reverse",
  },
});

const TheDropdownContent = styled("div", {
  base: {
    ...utility.shadowXl,
    zIndex: 10,
    position: "absolute",
    width: theme.space[32],
    borderRadius: theme.borderRadius,
    backgroundColor: theme.color.Surface0,
    border: `${theme.space.px} solid ${theme.color.Overlay0}`,
    marginTop: theme.space[1],
    selectors: {
      [`${TheDropdown}[open] &`]: {
        animation: `${appearAnimation} 0.1s`,
      },
      [`${TheDropdownEnd} &`]: {},
    },
  },
});

type DropdownContentProps = {
  end?: boolean;
} & JSX.HTMLAttributes<HTMLDivElement>;

export const DropdownContent: ParentComponent<DropdownContentProps> = (
  props
) => (
  <Show
    when={props.end}
    fallback={
      <TheDropdownContent {...props}>{props.children}</TheDropdownContent>
    }
  >
    <TheDropdownEnd>
      <TheDropdownContent {...props}>{props.children}</TheDropdownContent>
    </TheDropdownEnd>
  </Show>
);
