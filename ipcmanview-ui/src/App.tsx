import { JSX, ParentComponent, Component } from "solid-js";
import { A, AnchorProps, Route, Routes, useNavigate } from "@solidjs/router";
import {
  RiBuildingsHome5Line,
  RiDesignFocus2Line,
  RiSystemLogoutBoxRLine,
  RiUserAccountCircleFill,
  RiUserAdminFill,
} from "solid-icons/ri";

const Root: ParentComponent = (props) => (
  <div class="flex h-screen w-screen flex-col">{props.children}</div>
);

const CHIP_CLASS =
  "flex rounded-xl text-primary-content hover:bg-primary-content hover:text-primary-focus";
const CHIP_ACTIVE_CLASS = "bg-primary-content text-primary-focus";
const CHIP_INACTIVE_CLASS = "text-primary-content";

const Chip: ParentComponent<{ active?: boolean }> = (props) => (
  <div
    class={
      CHIP_CLASS +
      " [&>*]:rounded-xl [&>*]:p-1 " +
      (props.active ? CHIP_ACTIVE_CLASS : CHIP_INACTIVE_CLASS)
    }
  >
    {props.children}
  </div>
);

const LinkChip: ParentComponent<
  Omit<AnchorProps, "class" | "inactiveClass" | "activeClass">
> = (props) => (
  <A
    {...props}
    class={CHIP_CLASS + " p-1"}
    activeClass={CHIP_ACTIVE_CLASS}
    inactiveClass={CHIP_INACTIVE_CLASS}
  >
    {props.children}
  </A>
);

const Icon: ParentComponent = (props) => (
  <div class="[&>*]:h-6 [&>*]:w-6">{props.children}</div>
);

const Header: ParentComponent = (props) => (
  <header class="flex h-12 justify-between gap-2 bg-primary-focus p-2">
    {props.children}
  </header>
);

const HeaderTextLogo: ParentComponent = (props) => (
  <div class="overflow-hidden text-ellipsis whitespace-nowrap text-2xl font-bold text-primary-content">
    {props.children}
  </div>
);

const HeaderEnd: ParentComponent = (props) => (
  <div class="flex gap-2">{props.children}</div>
);

const Content: ParentComponent = (props) => (
  <div class="flex h-full flex-col overflow-hidden sm:flex-row">
    {props.children}
  </div>
);

const ContentNav: ParentComponent = (props) => (
  <div>
    <nav class="flex h-12 w-full flex-row justify-between gap-1 bg-primary p-2 sm:h-full sm:w-12 sm:flex-col">
      {props.children}
    </nav>
  </div>
);

const ContentNavStart: ParentComponent<{ end?: JSX.Element }> = (props) => (
  <div class="flex gap-1 overflow-auto" style="flex-direction: inherit;">
    {props.children}
  </div>
);

const ContentBody: ParentComponent = (props) => (
  <div class="h-full w-full overflow-auto p-4">{props.children}</div>
);

import { usePb } from "~/data/pb";
import { ADMIN_PANEL_URL } from "~/data/utils";

import Home from "~/pages/Home";
import Profile from "~/pages/Profile";
import Stations from "~/pages/Stations";
import StationsShow from "~/pages/Stations/Show";
import ThemeSwitcher from "~/ui/ThemeSwitcher";

export const App: Component = () => {
  const pb = usePb();

  const navigate = useNavigate();
  const logout = () => {
    pb.authStore.clear();
    navigate("/", { replace: true });
  };

  return (
    <Root>
      <Header>
        <HeaderTextLogo>IPCManView</HeaderTextLogo>
        <HeaderEnd>
          <LinkChip href="/profile">
            <Icon>
              <RiUserAccountCircleFill />
            </Icon>
          </LinkChip>
          <Chip>
            <button onClick={logout} title="Logout">
              <Icon>
                <RiSystemLogoutBoxRLine />
              </Icon>
            </button>
          </Chip>
          <Chip>
            <ThemeSwitcher />
          </Chip>
        </HeaderEnd>
      </Header>
      <Content>
        <ContentNav>
          <ContentNavStart>
            <LinkChip href="/" title="Home" end>
              <Icon>
                <RiBuildingsHome5Line />
              </Icon>
            </LinkChip>
            <LinkChip href="/stations" title="Stations">
              <Icon>
                <RiDesignFocus2Line />
              </Icon>
            </LinkChip>
          </ContentNavStart>
          <Chip>
            <a href={ADMIN_PANEL_URL} title="Admin Panel">
              <Icon>
                <RiUserAdminFill />
              </Icon>
            </a>
          </Chip>
        </ContentNav>
        <ContentBody>
          <Routes>
            <Route path="/" component={Home} />
            <Route path="/profile" component={Profile} />
            <Route path="/stations" component={Stations} />
            <Route path="/stations/:stationId" component={StationsShow} />
          </Routes>
        </ContentBody>
      </Content>
    </Root>
  );
};

export default App;
