import { A } from "@solidjs/router";
import { For, ParentComponent } from "solid-js";

export type BreadcrumbCrumb = { href: string; text: string };

type BreadcrumbProps = {
  crumbs: Array<BreadcrumbCrumb>;
};

const Breadcrumb: ParentComponent<BreadcrumbProps> = (props) => (
  <div class="flex flex-col gap-4">
    <nav class="flex gap-2">
      <For each={props.crumbs}>
        {(item) => (
          <>
            <A
              href={item.href}
              inactiveClass="text-link hover:text-ship-950"
              end
            >
              {item.text}
            </A>
            <div class="text-ship-300 last:hidden">/</div>
          </>
        )}
      </For>
    </nav>
    {props.children}
  </div>
);

export default Breadcrumb;
