import clsx from "clsx";
import { Component, JSX, mergeProps, Show, splitProps } from "solid-js";
import InputError from "./InputError";

type FormTextInputProps = {
  loading?: boolean;
  error?: string;
  label?: string;
} & JSX.InputHTMLAttributes<HTMLInputElement>;

const FormTextInput: Component<FormTextInputProps> = (props) => {
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
        class={clsx("rounded", props.error && "border-danger", props.class)}
        disabled={props.loading}
      />
      <InputError error={props.error} />
    </>
  );
};

export default FormTextInput;
