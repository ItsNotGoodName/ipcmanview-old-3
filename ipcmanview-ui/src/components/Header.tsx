import { Component, createSignal } from "solid-js";
import { RiUserAccountCircleFill } from "solid-icons/ri";
import clsx from "clsx";
import {
  Menu,
  MenuContent,
  MenuItem,
  MenuPositioner,
  MenuTrigger,
} from "@ark-ui/solid";
import { Portal } from "solid-js/web";
import { useNavigate, useLocation } from "@solidjs/router";

type HeaderProps = {
  class?: string;
  onLogout?: () => void;
};

const Header: Component<HeaderProps> = (props) => {
  const location = useLocation();
  const navigate = useNavigate();
  const isProfileRoute = () => location.pathname.startsWith("/profile");
  const [isOpen, setOpen] = createSignal(false);

  const onSelect: ((details: { value: string }) => void) | undefined = (id) => {
    switch (id.value) {
      case "profile":
        navigate("/profile");
        break;
      case "logout":
        props.onLogout && props.onLogout();
        break;
    }
  };

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
          onSelect={onSelect}
          onOpen={() => setOpen(true)}
          onClose={() => setOpen(false)}
          closeOnSelect
        >
          <MenuTrigger>
            <button
              title="User"
              class="m-auto rounded-xl p-2 text-ship-50 hover:bg-ship-50 hover:text-ship-950"
              classList={{
                "bg-ship-50 text-ship-950": isOpen() || isProfileRoute(),
              }}
            >
              <RiUserAccountCircleFill class="h-6 w-6" aria-hidden="true" />
            </button>
          </MenuTrigger>
          <Portal>
            <MenuPositioner>
              <MenuContent class="w-32 space-y-1 rounded-lg bg-ship-500 p-1 shadow">
                <MenuItem id="profile">
                  <button
                    class={clsx(
                      "flex w-full rounded p-1",
                      isProfileRoute()
                        ? "bg-ship-50 text-ship-950"
                        : "text-ship-50 hover:bg-ship-50 hover:text-ship-950"
                    )}
                  >
                    Profile
                  </button>
                </MenuItem>
                <MenuItem id="logout">
                  <button class="flex w-full rounded p-1 text-ship-50 hover:bg-ship-50 hover:text-ship-950">
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
