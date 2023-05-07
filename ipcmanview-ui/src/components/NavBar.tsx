import clsx from "clsx";
import { A } from "@solidjs/router";
import { Component } from "solid-js";
import { BiSolidCctv } from "solid-icons/bi";
import { RiBuildingsHome5Line } from "solid-icons/ri";
import { FaSolidSitemap } from "solid-icons/fa";

type NavBarProps = {
  class?: string;
};

const NavBar: Component<NavBarProps> = (props) => {
  return (
    <nav
      class={clsx(
        "flex gap-1 overflow-auto bg-blue-600 p-2 shadow",
        props.class
      )}
    >
      <div class="flex">
        <A
          href="/"
          class="m-auto rounded-xl p-2"
          inactiveClass="text-white hover:text-black hover:bg-white"
          activeClass="bg-white text-gray-900"
          end
        >
          <RiBuildingsHome5Line class="h-6 w-6" />
        </A>
      </div>

      <div class="flex">
        <A
          href="/sites"
          class="m-auto rounded-xl p-2"
          inactiveClass="text-white hover:text-black hover:bg-white"
          activeClass="bg-white text-gray-900"
          end
        >
          <FaSolidSitemap class="h-6 w-6" />
        </A>
      </div>

      <div class="flex">
        <A
          href="/cameras"
          class="m-auto rounded-xl p-2"
          inactiveClass="text-white hover:text-black hover:bg-white"
          activeClass="bg-white text-gray-900"
          end
        >
          <BiSolidCctv class="h-6 w-6" />
        </A>
      </div>
    </nav>
  );
};

export default NavBar;
