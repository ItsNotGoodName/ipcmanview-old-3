import clsx from "clsx";
import { JSX, ParentComponent, Show } from "solid-js";

type CardProps = {
  children: JSX.Element;
  title?: string;
  class?: string;
};

const Card: ParentComponent<CardProps> = (props) => {
  return (
    <div
      class={clsx(
        "flex flex-1 flex-col rounded p-2 shadow shadow-ship-300",
        props.class
      )}
    >
      <Show when={props.title}>
        <h1 class="mx-auto text-xl">{props.title}</h1>
      </Show>
      <div class="rounded p-2">{props.children}</div>
    </div>
  );
};

export default Card;
