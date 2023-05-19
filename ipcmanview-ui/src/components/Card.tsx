import clsx from "clsx";
import { JSX, ParentComponent, Show } from "solid-js";

type CardProps = {
  children: JSX.Element;
  right?: JSX.Element;
  title?: JSX.Element;
  sub?: JSX.Element;
  class?: string;
};

const Card: ParentComponent<CardProps> = (props) => {
  return (
    <div
      class={clsx("flex flex-col rounded shadow shadow-ship-300", props.class)}
    >
      <Show when={props.title || props.sub || props.right}>
        <div class="flex gap-4 rounded-t bg-ship-600 px-4 py-2 text-ship-50">
          <div class="flex-1">
            <Show when={props.title}>
              <div class="text-lg font-bold">{props.title}</div>
            </Show>
            <Show when={props.sub}>
              <div>{props.sub}</div>
            </Show>
          </div>
          <Show when={props.right}>
            <div>{props.right}</div>
          </Show>
        </div>
      </Show>
      {props.children}
    </div>
  );
};

export default Card;

export const CardBody: ParentComponent = (props) => {
  return <div class="p-4">{props.children}</div>;
};
