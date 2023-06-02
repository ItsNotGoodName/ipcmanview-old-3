import { JSX, ParentComponent, Show, splitProps } from "solid-js";
import Spinner from "./Spinner";

type ButtonProps = {
  loading?: boolean;
  children: JSX.Element;
} & Omit<JSX.ButtonHTMLAttributes<HTMLButtonElement>, "class">;

const Button: ParentComponent<Omit<ButtonProps, "disabled">> = (props) => {
  const [, other] = splitProps(props, ["loading", "children"]);

  return (
    <button
      {...other}
      class="flex w-full gap-1 truncate rounded bg-ship-500 p-2 text-ship-50 hover:bg-ship-600"
      disabled={props.loading}
    >
      <Show when={props.loading}>
        <div class="h-full">
          <Spinner />
        </div>
      </Show>
      {props.children}
    </button>
  );
};

export default Button;
