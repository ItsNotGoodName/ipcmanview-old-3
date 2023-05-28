import { JSX, ParentComponent, Show } from "solid-js";

export const Card: ParentComponent = (props) => (
  <div class="overflow-x-auto rounded border border-ship-600 shadow">
    {props.children}
  </div>
);

type CardHeaderProps = {
  title: JSX.Element;
  sub?: JSX.Element;
  right?: JSX.Element;
};

export const CardHeader: ParentComponent<CardHeaderProps> = (props) => (
  <div class="overflow-x-hidden bg-ship-600 text-ship-50">
    <div class="flex gap-4 px-4 py-2">
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
);

export const CardBody: ParentComponent = (props) => (
  <div class="overflow-x-auto p-4">{props.children}</div>
);
