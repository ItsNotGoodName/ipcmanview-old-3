import { Component, Show } from "solid-js";

type InputErrorProps = {
  error?: string;
};

const InputError: Component<InputErrorProps> = (props) => (
  <Show when={props.error}>
    <div class="text-danger-100">{props.error}</div>
  </Show>
);

export default InputError;
