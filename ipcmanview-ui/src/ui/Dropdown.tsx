import clsx from "clsx";
import { Accessor, Component, createSignal, JSX, splitProps } from "solid-js";

const Dropdown: Component<
  {
    children: (props: { open: Accessor<boolean> }) => JSX.Element;
  } & Omit<
    JSX.HTMLAttributes<HTMLDetailsElement>,
    "children" | "ref" | "onToggle" | "onFocusOut" | "onClick"
  >
> = (props) => {
  const [, other] = splitProps(props, ["children", "class"]);
  const [open, setOpen] = createSignal(false);
  let det: HTMLDetailsElement;

  return (
    <details
      class={clsx("dropdown", props.class)}
      {...other}
      ref={det!}
      onToggle={() => {
        setOpen(det.open);
      }}
      onClick={(e) => {
        if ((e.target as HTMLElement).dataset?.close) det.open = false;
      }}
      onFocusOut={(e) => {
        if (!e.relatedTarget || !det.contains(e.relatedTarget as Node))
          det.open = false;
      }}
    >
      <props.children open={open} />
    </details>
  );
};

export default Dropdown;
