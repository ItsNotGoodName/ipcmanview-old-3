import { style } from "@macaron-css/core";
import { styled } from "@macaron-css/solid";
import { A, LinkProps, Route, Routes } from "@solidjs/router";
import { RiSystemMore2Line } from "solid-icons/ri";
import { Component, Match, Switch } from "solid-js";

import { useStationApiRecord } from "~/data/station";
import { Dropdown, DropdownContent, DropdownSummary } from "~/ui/Dropdown";
import { IconSpinner } from "~/ui/Icon";
import { Menu, menuChildClass } from "~/ui/Menu";
import { theme } from "~/ui/theme";
import { utility } from "~/ui/utility";
import { StationCameras } from "./Cameras";
import { Drawer } from "./drawer";
import { StationFiles } from "./Files";
import { StationHome } from "./Home";

const Root = styled("div", {
  base: {
    display: "flex",
    height: "100%",
    overflow: "hidden",
  },
});

const Content = styled("div", {
  base: {
    flex: "auto",
    overflow: "auto",
  },
});

const linkClass = style({
  color: theme.color.Text,
  borderRadius: theme.borderRadius,
  padding: theme.space[1],
  ":hover": {
    background: theme.color.Surface2,
  },
});

const activeLinkClass = style({
  color: theme.color.Mantle,
  background: theme.color.Mauve,
  ":hover": {
    background: theme.color.Mauve2,
  },
});

const Link: Component<LinkProps> = (props) => (
  <A {...props} class={linkClass} activeClass={activeLinkClass} />
);

const LinkTitle = styled("div", {
  base: {
    ...utility.textLine(),
  },
});

const Header = styled("div", {
  base: {
    display: "flex",
    justifyContent: "space-between",
    alignItems: "center",
    gap: theme.space[1],
    borderBottom: `${theme.space.px} solid ${theme.color.Overlay0}`,
    paddingBottom: theme.space[1],
    marginBottom: theme.space[1],
    height: theme.space[8],
  },
});

const HeaderTitle = styled("div", {
  base: {
    ...utility.textLine(),
    fontWeight: "bold",
  },
});

const Stack = styled("div", {
  base: {
    ...utility.stack("1"),
    padding: theme.space[2],
  },
});

const settingsButtonClass = style({
  display: "flex",
  alignItems: "center",
  borderRadius: theme.borderRadius,
  ":hover": {
    background: theme.color.Surface2,
  },
});

const settingsButtonIconClass = style({
  ...utility.size("6"),
});

export const Station: Component = () => {
  const station = useStationApiRecord();

  return (
    <Root>
      <Drawer>
        <Stack>
          <Header>
            <HeaderTitle title={station.data?.name}>
              <Switch>
                <Match when={station.isLoading}>
                  <IconSpinner />
                </Match>
                <Match when={station.isSuccess}>{station.data!.name}</Match>
              </Switch>
            </HeaderTitle>
            <Dropdown>
              {() => (
                <>
                  <DropdownSummary class={settingsButtonClass}>
                    <RiSystemMore2Line class={settingsButtonIconClass} />
                  </DropdownSummary>
                  <DropdownContent end={true}>
                    <Menu>
                      <div class={menuChildClass}>Edit</div>
                    </Menu>
                  </DropdownContent>
                </>
              )}
            </Dropdown>
          </Header>
          <Link href="" end>
            <LinkTitle>Home</LinkTitle>
          </Link>
          <Link href="cameras">
            <LinkTitle>Cameras</LinkTitle>
          </Link>
          <Link href="files">
            <LinkTitle>Files</LinkTitle>
          </Link>
        </Stack>
      </Drawer>
      <Content>
        <Routes>
          <Route path="" component={StationHome} />
          <Route path="cameras" component={StationCameras} />
          <Route path="files" component={StationFiles} />
        </Routes>
      </Content>
    </Root>
  );
};
