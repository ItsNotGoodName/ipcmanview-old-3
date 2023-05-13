import clsx from "clsx";
import { Component } from "solid-js";
import { BiSolidCctv } from "solid-icons/bi";
import NavButton from "./NavButton";
import { RiBuildingsHome5Line, RiUserAdminFill } from "solid-icons/ri";
import { adminPageUrl } from "../pb";

type NavBarProps = {
  class?: string;
};

const NavBar: Component<NavBarProps> = (props) => {
  return (
    <nav
      class={clsx(
        "flex justify-between gap-1 bg-ship-600 p-2 shadow shadow-ship-300",
        props.class
      )}
    >
      <div class="flex gap-1 overflow-auto" style="flex-direction: inherit;">
        <NavButton title="Home" href="/" end>
          <RiBuildingsHome5Line class="h-6 w-6" />
        </NavButton>

        <NavButton title="Cameras" href="/cameras">
          <BiSolidCctv class="h-6 w-6" />
        </NavButton>
      </div>

      <a
        class="flex w-fit rounded-xl text-ship-50 hover:bg-ship-50 hover:text-ship-950"
        title="Admin"
        href={adminPageUrl}
      >
        <div class="m-2 inline-flex">
          <RiUserAdminFill class="h-6 w-6" />
        </div>
      </a>
    </nav>
  );
};

export default NavBar;
