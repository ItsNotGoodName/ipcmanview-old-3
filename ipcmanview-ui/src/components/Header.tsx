import { Component, createSignal, JSX } from "solid-js";
import { RiUserAccountCircleFill } from "solid-icons/ri";
import clsx from "clsx";
import {
  Menu,
  MenuArrow,
  MenuArrowTip,
  MenuContent,
  MenuContextTrigger,
  MenuItem,
  MenuItemGroup,
  MenuItemGroupLabel,
  MenuOptionItem,
  MenuPositioner,
  MenuSeparator,
  MenuTrigger,
  MenuTriggerItem,
} from "@ark-ui/solid";
import { Portal } from "solid-js/web";
import { A, useLocation } from "@solidjs/router";

type HeaderProps = {
  class?: string;
  onLogout?: JSX.EventHandlerUnion<HTMLButtonElement, MouseEvent>;
};

const Header: Component<HeaderProps> = (props) => {
  const location = useLocation();
  const isProfileRoute = () => location.pathname.startsWith("/profile");
  const [isOpen, setOpen] = createSignal(false);

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
        <Menu
          onOpen={() => setOpen(true)}
          onClose={() => setOpen(false)}
          closeOnSelect
        >
          <MenuTrigger
            class="m-auto rounded-xl p-2 text-ship-50 hover:bg-ship-50 hover:text-ship-950"
            classList={{
              "bg-ship-50 text-ship-950": isOpen() || isProfileRoute(),
            }}
          >
            <RiUserAccountCircleFill class="h-6 w-6" aria-hidden="true" />
          </MenuTrigger>
          <Portal>
            <MenuPositioner>
              <MenuContent class="w-32 space-y-1 rounded-lg bg-ship-50 p-1 shadow">
                <MenuItem id="profile">
                  <A
                    class="flex w-full rounded p-1 hover:bg-ship-500 hover:text-ship-50"
                    classList={{
                      "bg-ship-500 text-ship-50": isProfileRoute(),
                    }}
                    href="/profile"
                  >
                    Profile
                  </A>
                </MenuItem>
                <MenuItem id="logout">
                  <button
                    class="flex w-full rounded p-1 hover:bg-ship-500 hover:text-ship-50"
                    onClick={props.onLogout}
                  >
                    Logout
                  </button>
                </MenuItem>
              </MenuContent>
            </MenuPositioner>
          </Portal>
        </Menu>
      </div>
    </header>
  );
};

export default Header;
