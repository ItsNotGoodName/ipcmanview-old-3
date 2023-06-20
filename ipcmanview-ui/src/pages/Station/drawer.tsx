import { style } from "@macaron-css/core";
import { styled } from "@macaron-css/solid";
import { RiSystemSideBarLine } from "solid-icons/ri";
import { onCleanup, ParentComponent } from "solid-js";

import { maxScreen, minScreen, theme } from "~/ui/theme";
import { utility } from "~/ui/utility";

const TheDrawer = styled("div", {});

const TheDrawerButton = styled("div", {
  base: {
    ...utility.shadowXl,
    display: "flex",
    alignContent: "center",
    margin: theme.space[2],
    padding: theme.space[2],
    bottom: 0,
    borderRadius: "100%",
    position: "absolute",
    border: `${theme.space.px} solid ${theme.color.Overlay0}`,
    background: theme.color.Base,
    zIndex: 5,
    "@media": {
      [minScreen.md]: {
        display: "none",
      },
    },
    ":hover": {
      background: theme.color.Surface2,
    },
  },
});

const Relative = styled("div", {
  base: {
    height: "100%",
    "@media": {
      [maxScreen.md]: {
        position: "relative",
      },
    },
  },
});

const RelativePositioner = styled("div", {
  base: {
    width: theme.space[40],
    height: "100%",
    transition: "width 0.25s",
    overflowY: "auto",
    overflowX: "hidden",
    "@media": {
      [maxScreen.md]: {
        position: "absolute",
        width: theme.space[0],
        paddingRight: theme.space[4],
        selectors: {
          [`${TheDrawer}[data-open] &`]: {
            width: theme.space[40],
            paddingRight: theme.space[0],
          },
        },
      },
    },
  },
});

const TheDrawerContent = styled("div", {
  base: {
    borderRight: `${theme.space.px} solid ${theme.color.Overlay0}`,
    background: theme.color.Surface0,
    height: "100%",
    overflowX: "hidden",
  },
});

export const Drawer: ParentComponent = (props) => {
  let drawer: HTMLDivElement;

  let listening = false;
  const onClick = (ev: MouseEvent) => {
    if (!drawer.contains(ev.target as Node)) {
      delete drawer.dataset.open;
      document.removeEventListener("click", onClick);
      listening = false;
    }
  };
  const startListening = () => {
    if (!listening) {
      document.addEventListener("click", onClick);
      listening = true;
    }
  };
  onCleanup(() => {
    if (listening) document.removeEventListener("click", onClick);
  });

  const openDrawer = () => {
    drawer.dataset.open = "";
    startListening();
  };
  const toggleDrawer = () => {
    if (drawer.dataset.open == "") {
      delete drawer.dataset.open;
    } else {
      openDrawer();
    }
  };

  return (
    <TheDrawer ref={drawer!}>
      <TheDrawerButton onClick={toggleDrawer}>
        <RiSystemSideBarLine class={style({ ...utility.size("6") })} />
      </TheDrawerButton>
      <Relative>
        <RelativePositioner onClick={openDrawer}>
          <TheDrawerContent>{props.children}</TheDrawerContent>
        </RelativePositioner>
      </Relative>
    </TheDrawer>
  );
};
