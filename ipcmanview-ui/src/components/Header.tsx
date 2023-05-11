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
        "flex justify-between gap-1 bg-ship-700 p-2 shadow shadow-ship-300",
        props.class
      )}
    >
      <div class="flex gap-1 overflow-clip">
        <h1 class="my-auto text-xl font-bold text-ship-50">IPCManView</h1>
      </div>

      <div class="flex">
        <button
          class="m-auto cursor-pointer rounded-xl p-2 text-ship-50 hover:bg-ship-50 hover:text-ship-950"
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
