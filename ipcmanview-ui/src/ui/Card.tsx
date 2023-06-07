import { JSX, ParentComponent, Show } from "solid-js";

export const Card: ParentComponent = (props) => (
  <div class="overflow-x-auto rounded-lg border border-base-300 bg-base-200 shadow">
    {props.children}
  </div>
);

type CardHeaderProps = {
  sub?: JSX.Element;
  right?: JSX.Element;
};

export const CardHeader: ParentComponent<CardHeaderProps> = (props) => (
  <div class="overflow-x-hidden bg-secondary-focus text-secondary-content">
    <div class="flex gap-4 px-4 py-2">
      <div class="flex-1 truncate">
        <div class="text-lg font-bold">{props.children}</div>
        <Show when={props.sub}>
          <div>{props.sub}</div>
        </Show>
      </div>
      <Show when={props.right}>
        <div class="flex">{props.right}</div>
      </Show>
    </div>
  </div>
);

export const CardBody: ParentComponent = (props) => (
  <div class="overflow-x-auto p-4">{props.children}</div>
);
