import { Component } from "solid-js";
import { RiUserAccountCircleFill } from "solid-icons/ri";
import clsx from "clsx";
import pb from "../pb";
import {
  Menu,
  MenuItem,
  Popover,
  PopoverButton,
  PopoverPanel,
  Transition,
} from "solid-headless";
import { A, useLocation } from "@solidjs/router";

type HeaderProps = {
  class?: string;
};

const Header: Component<HeaderProps> = (props) => {
  const location = useLocation();
  const isProfileRoute = () => location.pathname.startsWith("/profile");

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
        <Popover defaultOpen={false} class="relative">
          {({ isOpen, setState }) => (
            <>
              <PopoverButton
                class="m-auto rounded-xl p-2 text-ship-50 hover:bg-ship-50 hover:text-ship-950"
                classList={{
                  "bg-ship-50 text-ship-950": isOpen() || isProfileRoute(),
                }}
              >
                <RiUserAccountCircleFill class="h-6 w-6" aria-hidden="true" />
              </PopoverButton>
              <Transition
                show={isOpen()}
                enter="transition duration-200"
                enterFrom="opacity-0 -translate-y-1 scale-50"
                enterTo="opacity-100 translate-y-0 scale-100"
                leave="transition duration-150"
                leaveFrom="opacity-100 translate-y-0 scale-100"
                leaveTo="opacity-0 -translate-y-1 scale-50"
              >
                <PopoverPanel
                  unmount={false}
                  class="absolute right-0 z-10 mt-2"
                >
                  <Menu class="flex w-32 flex-col space-y-1 overflow-hidden rounded-lg bg-ship-50 p-1 shadow">
                    <MenuItem
                      as="button"
                      class="flex rounded p-1 text-left hover:bg-ship-500 hover:text-ship-50"
                      classList={{
                        "bg-ship-500 text-ship-50": isProfileRoute(),
                      }}
                    >
                      <A
                        class="w-full"
                        href="/profile"
                        onClick={() => setState(false)}
                      >
                        Profile
                      </A>
                    </MenuItem>
                    <MenuItem
                      as="button"
                      class="rounded p-1 text-left hover:bg-ship-500 hover:text-ship-50"
                      onClick={() => {
                        pb.authStore.clear();
                      }}
                    >
                      Logout
                    </MenuItem>
                  </Menu>
                </PopoverPanel>
              </Transition>
            </>
          )}
        </Popover>
      </div>
    </header>
  );
};

export default Header;
