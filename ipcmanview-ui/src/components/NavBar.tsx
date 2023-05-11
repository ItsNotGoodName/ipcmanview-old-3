import clsx from "clsx";
import { A } from "@solidjs/router";
import { Component } from "solid-js";
import { BiSolidCctv } from "solid-icons/bi";
import { RiBuildingsHome5Line } from "solid-icons/ri";

type NavBarProps = {
  class?: string;
};

const NavBar: Component<NavBarProps> = (props) => {
  return (
    <nav
      class={clsx(
        "flex gap-1 overflow-auto bg-ship-600 p-2 shadow shadow-ship-300",
        props.class
      )}
    >
      <div class="flex">
        <A
          href="/"
          class="m-auto rounded-xl p-2"
          inactiveClass="text-ship-50 hover:text-ship-950 hover:bg-ship-50"
          activeClass="bg-ship-50 text-ship-950"
          end
        >
          <RiBuildingsHome5Line class="h-6 w-6" />
        </A>
      </div>

      <div class="flex">
        <A
          href="/cameras"
          class="m-auto rounded-xl p-2"
          inactiveClass="text-ship-50 hover:text-ship-950 hover:bg-ship-50"
          activeClass="bg-ship-50 text-ship-950"
          end
        >
          <BiSolidCctv class="h-6 w-6" />
        </A>
      </div>
    </nav>
  );
};

export default NavBar;
