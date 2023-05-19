import { A, AnchorProps } from "@solidjs/router";
import { ParentComponent } from "solid-js";

const NavLink: ParentComponent<
  Omit<AnchorProps, "class" | "inactiveClass" | "activeClass">
> = (props) => {
  return (
    <A
      {...props}
      class="flex w-fit rounded-xl hover:bg-ship-100"
      inactiveClass="text-ship-50 hover:text-ship-950"
      activeClass="bg-ship-50 text-ship-950"
    >
      <div class="m-2 inline-flex">{props.children}</div>
    </A>
  );
};

export default NavLink;
