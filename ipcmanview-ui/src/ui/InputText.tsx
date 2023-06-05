import { Component, JSX, mergeProps, Show, splitProps } from "solid-js";
import InputError from "./InputError";

type InputTextProps = {
  loading?: boolean;
  error?: string;
  label?: string;
} & Omit<JSX.InputHTMLAttributes<HTMLInputElement>, "class">;

const InputText: Component<InputTextProps> = (props) => {
  const [, other] = splitProps(mergeProps({ type: "text" }, props), [
    "loading",
    "error",
    "label",
  ]);

  return (
    <div class="form-control gap-2">
      <Show when={props.label}>
        <label class="text mr-2 font-bold" for={props.name}>
          {props.label}{" "}
          {props.required && <span class="ml-1 text-error">*</span>}
        </label>
      </Show>
      <input
        {...other}
        class="input-bordered input w-full"
        classList={{ "input-error": !!props.error }}
        disabled={props.loading}
      />
      <InputError error={props.error} />
    </div>
  );
};

export default InputText;
