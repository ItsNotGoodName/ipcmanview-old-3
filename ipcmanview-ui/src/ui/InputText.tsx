import { styled } from "@macaron-css/solid";
import { Component, JSX, mergeProps, Show, splitProps } from "solid-js";

import ErrorText from "./ErrorText";
import { theme } from "./theme";

const Control = styled("div", {
  base: {
    display: "flex",
    flexDirection: "column",
    gap: theme.space[2],
  },
});

const Label = styled("label", {
  base: {
    fontWeight: "bold",
  },
});

const Input = styled("input", {
  base: {
    borderRadius: theme.borderRadius,
    border: "none",
  },
  variants: {
    size: {
      small: {
        padding: theme.space["0.5"],
      },
      medium: {
        padding: theme.space[2],
      },
      large: {
        padding: theme.space[4],
      },
    },
    error: {
      true: {
        borderColor: theme.color.Red,
      },
    },
  },
  defaultVariants: {
    size: "medium",
  },
});

const Required = styled("span", {
  base: {
    color: theme.color.Red,
  },
});

type InputTextProps = {
  error?: string;
  label?: string;
  size?: "small" | "medium" | "large";
} & JSX.InputHTMLAttributes<HTMLInputElement>;

const InputText: Component<InputTextProps> = (props) => {
  const [, other] = splitProps(mergeProps({ type: "text" }, props), [
    "error",
    "label",
  ]);

  return (
    <Control>
      <Show when={props.label}>
        <Label for={props.name}>
          {props.label} {props.required && <Required>*</Required>}
        </Label>
      </Show>
      <Input {...other} error={!!props.error} />
      <Show when={props.error}>
        <ErrorText>{props.error}</ErrorText>
      </Show>
    </Control>
  );
};

export default InputText;
