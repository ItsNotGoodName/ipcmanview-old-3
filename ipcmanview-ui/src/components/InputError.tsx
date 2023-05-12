import { Component, Show } from "solid-js";

type InputErrorProps = {
  error?: string;
};

const InputError: Component<InputErrorProps> = (props) => {
  return (
    <Show when={props.error}>
      <div class="text-danger">{props.error}</div>
    </Show>
  );
};

export default InputError;
