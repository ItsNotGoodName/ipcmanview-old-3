import { Component, JSX, mergeProps, Show, splitProps } from "solid-js";
import InputError from "./InputError";

type InputTextFragProps = {
  loading?: boolean;
  error?: string;
  label?: string;
} & Omit<JSX.InputHTMLAttributes<HTMLInputElement>, "class">;

const InputTextFrag: Component<InputTextFragProps> = (props) => {
  const [, other] = splitProps(mergeProps({ type: "text" }, props), [
    "loading",
    "error",
    "label",
  ]);

  return (
    <>
      <Show when={props.label}>
        <label class="mr-2 font-bold" for={props.name}>
          {props.label}{" "}
          {props.required && <span class="ml-1 text-danger-100">*</span>}
        </label>
      </Show>
      <input
        {...other}
        class="rounded"
        classList={{
          "border-danger-100": !!props.error,
          "opacity-80": props.loading,
        }}
        disabled={props.loading}
      />
      <InputError error={props.error} />
    </>
  );
};

export default InputTextFrag;
