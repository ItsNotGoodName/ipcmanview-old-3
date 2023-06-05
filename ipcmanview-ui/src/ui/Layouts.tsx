import { ParentComponent } from "solid-js";

export const CenterLayout: ParentComponent = (props) => (
  <div class="px-4 py-16">
    <div class="mx-auto flex max-w-sm flex-col gap-4">{props.children}</div>
  </div>
);
