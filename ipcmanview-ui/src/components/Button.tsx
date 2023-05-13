import clsx from "clsx";
import { JSX, ParentComponent, Show, splitProps } from "solid-js";
import { RiSystemLoader4Fill } from "solid-icons/ri";

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
        <div class="animate-spin">
          <RiSystemLoader4Fill class="h-6 w-6" />
        </div>
      </Show>
      {props.children}
    </button>
  );
};

export default Button;
