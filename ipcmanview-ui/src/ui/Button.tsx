import clsx from "clsx";
import { Component, JSX, Show, splitProps } from "solid-js";

type ButtonProps = {
  loading?: boolean;
  children: JSX.Element;
} & JSX.ButtonHTMLAttributes<HTMLButtonElement>;

const Button: Component<ButtonProps> = (props) => {
  const [, other] = splitProps(props, [
    "loading",
    "children",
    "class",
    "disabled",
  ]);

  return (
    <button
      {...other}
      class={clsx("no-animation btn", props.class)}
      disabled={props.loading || props.disabled}
    >
      <Show when={props.loading}>
        <span class="loading loading-spinner"></span>
      </Show>
      {props.children}
    </button>
  );
};

export default Button;
