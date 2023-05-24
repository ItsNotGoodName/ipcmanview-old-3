import clsx from "clsx";
import { JSX, ParentComponent, Show } from "solid-js";

type NormalCard = {
  class?: string;
};

const NormalCard: ParentComponent<NormalCard> = (props) => {
  return (
    <div
      class={clsx(
        "overflow-x-auto rounded border border-ship-300 shadow",
        props.class
      )}
    >
      {props.children}
    </div>
  );
};

type HeaderCardProps = {
  right?: JSX.Element;
  title?: JSX.Element;
  sub?: JSX.Element;
} & NormalCard;

const HeaderCard: ParentComponent<HeaderCardProps> = (props) => {
  return (
    <div class={clsx("flex flex-col", props.class)}>
      <div class="rounded-t bg-ship-600 text-ship-50">
        <div class="mx-4 my-2 flex gap-4">
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
      </div>
      <div class="overflow-x-auto rounded-b border-x border-b border-ship-300 shadow">
        {props.children}
      </div>
    </div>
  );
};

const Body: ParentComponent = (props) => {
  return <div class="m-4">{props.children}</div>;
};

export default {
  NormalCard,
  HeaderCard,
  Body,
};
