import { Component, Show } from "solid-js";

type InputErrorProps = {
  error?: string;
};

const InputError: Component<InputErrorProps> = (props) => (
  <Show when={props.error}>
    <div class="text-error">{props.error}</div>
  </Show>
);

export default InputError;
