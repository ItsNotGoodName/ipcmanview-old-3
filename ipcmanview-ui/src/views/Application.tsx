import { Component } from "solid-js";
import {
  A,
  Route,
  Routes,
  useLocation,
  useNavigate,
  useParams,
} from "@solidjs/router";
import {
  RiBuildingsHome5Line,
  RiDesignFocus2Line,
  RiSystemLogoutBoxRLine,
  RiUserAccountCircleFill,
  RiUserAdminFill,
} from "solid-icons/ri";
import { styled } from "@macaron-css/solid";
import { CSSProperties, style } from "@macaron-css/core";

import { ADMIN_PANEL_URL, nameToInitials } from "~/data/utils";
import { PbStationApiProvider, usePb, usePbUser } from "~/data/pb";
import { minScreen, theme } from "~/ui/theme";

const Root = styled("div", {
  base: {
    display: "flex",
    flexDirection: "column",
    height: "100%",
  },
});

const Header = styled("div", {
  base: {
    display: "flex",
    height: theme.space[11],
    gap: theme.space[2],
    justifyContent: "space-between",
    padding: theme.space[2],
    background: theme.color.Crust,
    borderBottom: `${theme.space.px} solid ${theme.color.Overlay0}`,
  },
});

const HeaderText = styled("div", {
  base: {
    overflow: "hidden",
    textOverflow: "ellipsis",
    whiteSpace: "nowrap",
    fontSize: "x-large",
  },
});

const HeaderEnd = styled("div", {
  base: {
    display: "flex",
    gap: theme.space[2],
  },
});

const Content = styled("div", {
  base: {
    display: "flex",
    flexDirection: "column",
    height: "100%",
    overflow: "hidden",
    "@media": {
      [minScreen.md]: {
        flexDirection: "row",
      },
    },
  },
});

const ContentNav = styled("div", {
  base: {
    display: "flex",
    gap: theme.space[1],
    padding: theme.space[2],
    background: theme.color.Mantle,
    justifyContent: "space-between",
    height: theme.space[11],
    borderBottom: `${theme.space.px} solid ${theme.color.Overlay0}`,
    overflowX: "auto",
    "@media": {
      [minScreen.md]: {
        width: theme.space[11],
        flexDirection: "column",
        height: "100%",
        borderRight: `${theme.space.px} solid ${theme.color.Overlay0}`,
        borderBottom: "none",
      },
    },
  },
});

const ContentNavStart = styled("div", {
  base: {
    display: "flex",
    gap: "inherit",
    flexDirection: "inherit",
  },
});

const ContentBody = styled("div", {
  base: {
    width: "100%",
    height: "100%",
    overflow: "auto",
    padding: theme.space[4],
  },
});

const chipClass = style({
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
  color: theme.color.Text,
  ":active": {
    color: theme.color.Mauve,
    borderColor: theme.color.Mauve,
    fill: theme.color.Mauve,
  },
});

const activeChipClass = style({
  color: theme.color.Mauve,
  borderColor: theme.color.Mauve,
  fill: theme.color.Mauve,
});

const chipChildStyle = {
  height: theme.space[7],
  width: theme.space[7],
} satisfies CSSProperties;

const iconClass = style({
  ...chipChildStyle,
});

const avatarClass = style({
  ...chipChildStyle,
  border: `${theme.space.px} solid`,
  borderRadius: "100%",
  backgroundColor: theme.color.Surface0,
  textTransform: "uppercase",
  padding: theme.space["0.5"],
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  userSelect: "none",
});

import Dropdown, { DropdownButton, DropdownContent } from "~/ui/Dropdown";
import ThemeSwitcher from "~/ui/ThemeSwitcher";
import Button from "~/ui/Button";
import { Row, Stack } from "~/ui/utility";

import Home from "~/pages/Home";
import Profile from "~/pages/Profile";
import Stations from "~/pages/Stations";
import StationsShow from "~/pages/Stations/Show";

const Application: Component = () => {
  const pb = usePb();
  const navigate = useNavigate();

  const logout = () => {
    pb.authStore.clear();
    navigate("/", { replace: true });
  };

  return (
    <Root>
      <div>
        <Header>
          <HeaderText>IPCManView</HeaderText>
          <HeaderEnd>
            <ThemeSwitcher class={chipClass} iconClass={iconClass} />
            <Dropdown>
              {(props) => {
                const { user } = usePbUser();
                const location = useLocation();
                const active = () =>
                  props.open() || location.pathname == "/profile";

                return (
                  <>
                    <DropdownButton
                      class={chipClass}
                      classList={{ [activeChipClass]: active() }}
                    >
                      <div class={avatarClass}>
                        {nameToInitials(user().name)}
                      </div>
                    </DropdownButton>
                    <DropdownContent position="end">
                      <Stack gap={1}>
                        <A href="/profile" onclick={props.close}>
                          <Row gap={1}>
                            <RiUserAccountCircleFill />
                            Profile
                          </Row>
                        </A>
                        <Button onClick={logout} size="small" color="danger">
                          <Row gap={1}>
                            <RiSystemLogoutBoxRLine />
                            Logout
                          </Row>
                        </Button>
                      </Stack>
                    </DropdownContent>
                  </>
                );
              }}
            </Dropdown>
          </HeaderEnd>
        </Header>
      </div>
      <Content>
        <div>
          <ContentNav>
            <ContentNavStart>
              <A
                href="/"
                title="Home"
                end
                class={chipClass}
                activeClass={activeChipClass}
              >
                <RiBuildingsHome5Line class={iconClass} />
              </A>
              <A
                href="/stations"
                title="Stations"
                class={chipClass}
                activeClass={activeChipClass}
              >
                <RiDesignFocus2Line class={iconClass} />
              </A>
            </ContentNavStart>
            <a href={ADMIN_PANEL_URL} title="Admin Panel" class={chipClass}>
              <RiUserAdminFill class={iconClass} />
            </a>
          </ContentNav>
        </div>
        <ContentBody>
          <Routes>
            <Route path="/" component={Home} />
            <Route path="/profile" component={Profile} />
            <Route path="/stations" component={Stations} />
            <Route
              path="/stations/:stationId"
              component={() => {
                const { stationId } = useParams<{ stationId: string }>();
                return (
                  <PbStationApiProvider stationId={stationId}>
                    <StationsShow />
                  </PbStationApiProvider>
                );
              }}
            />
          </Routes>
        </ContentBody>
      </Content>
    </Root>
  );
};

export default Application;
