import { Component, Show } from "solid-js";

type ErrorTextProps = {
  error?: string;
};

const ErrorText: Component<ErrorTextProps> = (props) => (
  <Show when={props.error}>
    <div class="text-error">{props.error}</div>
  </Show>
);

export default ErrorText;
