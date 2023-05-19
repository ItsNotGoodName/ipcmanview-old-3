import clsx from "clsx";
import { JSX, ParentComponent, Show, splitProps } from "solid-js";
import Spinner from "./Spinner";

type ButtonProps = {
  loading?: boolean;
  children: JSX.Element;
} & JSX.ButtonHTMLAttributes<HTMLButtonElement>;

const Button: ParentComponent<Omit<ButtonProps, "disabled">> = (props) => {
  const [, other] = splitProps(props, ["loading", "children", "class"]);

  return (
    <button
      {...other}
      class={clsx(
        "flex w-full gap-1 truncate rounded bg-ship-500 p-2 text-ship-50 hover:bg-ship-600",
        props.class
      )}
      disabled={props.loading}
    >
      <Show when={props.loading}>
        <Spinner />
      </Show>
      {props.children}
    </button>
  );
};

export default Button;
