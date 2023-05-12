import clsx from "clsx";
import { Component } from "solid-js";
import { BiSolidCctv } from "solid-icons/bi";
import NavButton from "./NavButton";
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
      <NavButton title="Home" href="/" end>
        <RiBuildingsHome5Line class="h-6 w-6" />
      </NavButton>

      <NavButton title="Cameras" href="/cameras">
        <BiSolidCctv class="h-6 w-6" />
      </NavButton>
    </nav>
  );
};

export default NavBar;
