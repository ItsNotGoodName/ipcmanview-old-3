import { Component } from "solid-js";
import { RiUserAccountCircleFill } from "solid-icons/ri";
import clsx from "clsx";
import pb from "../pb";

type HeaderProps = {
  class?: string;
};

const Header: Component<HeaderProps> = (props) => {
  return (
    <header
      class={clsx(
        "flex justify-between gap-1 bg-blue-700 p-2 shadow",
        props.class
      )}
    >
      <div class="flex gap-1 overflow-clip">
        <h1 class="my-auto text-xl font-bold text-white">IPCManView</h1>
      </div>

      <div class="flex">
        <button
          class="m-auto cursor-pointer rounded-xl p-2 text-white hover:bg-white hover:text-black"
          onClick={() => {
            pb.authStore.clear();
          }}
        >
          <RiUserAccountCircleFill class="h-6 w-6" />
        </button>
      </div>
    </header>
  );
};

export default Header;
