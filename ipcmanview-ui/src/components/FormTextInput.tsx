import clsx from "clsx";
import { Component, JSX, mergeProps, Show, splitProps } from "solid-js";

type FormTextInputProps = {
  loading?: boolean;
  error?: string;
} & JSX.InputHTMLAttributes<HTMLInputElement>;

const FormTextInput: Component<FormTextInputProps> = (props) => {
  const [, other] = splitProps(mergeProps({ type: "text" }, props), [
    "loading",
    "error",
    "class",
  ]);

  return (
    <>
      <input
        {...other}
        class={clsx("rounded", props.error && "border-danger", props.class)}
        disabled={props.loading}
      />
      <Show when={props.error}>
        <div class="text-danger">{props.error}</div>
      </Show>
    </>
  );
};

export default FormTextInput;
