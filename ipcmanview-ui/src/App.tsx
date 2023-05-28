import { JSX, ParentComponent, Component } from "solid-js";
import { A, AnchorProps, Route, Routes, useNavigate } from "@solidjs/router";
import { BiSolidLandmark } from "solid-icons/bi";
import {
  RiBuildingsHome5Line,
  RiSystemLogoutBoxRLine,
  RiUserAccountCircleFill,
  RiUserAdminFill,
} from "solid-icons/ri";

const Root: ParentComponent = (props) => (
  <div class="flex h-screen w-screen flex-col">{props.children}</div>
);

const CHIP_CLASS = "flex rounded-xl hover:bg-ship-50 hover:text-ship-950";
const CHIP_ACTIVE_CLASS = "bg-ship-50 text-ship-950";
const CHIP_INACTIVE_CLASS = "text-ship-50";

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

const ChipIcon: ParentComponent = (props) => (
  <div class="[&>*]:h-6 [&>*]:w-6">{props.children}</div>
);

const Header: ParentComponent = (props) => (
  <header class="flex h-12 justify-between gap-2 bg-ship-800 p-2 shadow shadow-ship-300">
    {props.children}
  </header>
);

const HeaderTextLogo: ParentComponent = (props) => (
  <div class="overflow-hidden text-ellipsis whitespace-nowrap text-2xl font-bold text-ship-50">
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
    <nav class="flex h-12 w-full flex-row justify-between gap-1 bg-ship-700 p-2 shadow shadow-ship-300 sm:h-full sm:w-12 sm:flex-col">
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

import { useAuthRefresh } from "./hooks";
import { usePb } from "./pb";
import { ADMIN_PANEL_URL } from "./utils";

import Home from "./pages/Home";
import Profile from "./pages/Profile";
import Stations from "./pages/Stations";
import StationsShow from "./pages/StationsShow";

export const App: Component = () => {
  const pb = usePb();
  useAuthRefresh(pb, { refetchOnWindowFocus: false });

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
            <ChipIcon>
              <RiUserAccountCircleFill />
            </ChipIcon>
          </LinkChip>
          <Chip>
            <button onClick={logout} title="Logout">
              <ChipIcon>
                <RiSystemLogoutBoxRLine />
              </ChipIcon>
            </button>
          </Chip>
        </HeaderEnd>
      </Header>
      <Content>
        <ContentNav>
          <ContentNavStart>
            <LinkChip href="/" title="Home" end>
              <ChipIcon>
                <RiBuildingsHome5Line />
              </ChipIcon>
            </LinkChip>
            <LinkChip href="/stations" title="Stations">
              <ChipIcon>
                <BiSolidLandmark />
              </ChipIcon>
            </LinkChip>
          </ContentNavStart>
          <Chip>
            <a href={ADMIN_PANEL_URL} title="Admin Panel">
              <ChipIcon>
                <RiUserAdminFill />
              </ChipIcon>
            </a>
          </Chip>
        </ContentNav>
        <ContentBody>
          <Routes>
            <Route path="/" component={Home} />
            <Route path="/profile" component={Profile} />
            <Route path="/stations" component={Stations} />
            <Route path="/stations/:id" component={StationsShow} />
          </Routes>
        </ContentBody>
      </Content>
    </Root>
  );
};

export default App;
