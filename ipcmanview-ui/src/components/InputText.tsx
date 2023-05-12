import clsx from "clsx";
import { Component, JSX, mergeProps, Show, splitProps } from "solid-js";
import InputError from "./InputError";

type InputTextProps = {
  loading?: boolean;
  error?: string;
  label?: string;
} & JSX.InputHTMLAttributes<HTMLInputElement>;

const InputText: Component<InputTextProps> = (props) => {
  const [, other] = splitProps(mergeProps({ type: "text" }, props), [
    "loading",
    "error",
    "class",
    "label",
  ]);

  return (
    <>
      <Show when={props.label}>
        <label class="font-bold" for={props.name}>
          {props.label}{" "}
          {props.required && <span class="ml-1 text-danger">*</span>}
        </label>
      </Show>
      <input
        {...other}
        class={clsx(
          "rounded",
          props.error && "border-danger",
          props.loading && "opacity-80",
          props.class
        )}
        disabled={props.loading}
      />
      <InputError error={props.error} />
    </>
  );
};

export default InputText;