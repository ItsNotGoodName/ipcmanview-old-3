import { style } from "@macaron-css/core";
import { styled } from "@macaron-css/solid";
import { Component } from "solid-js";
import { Button } from "~/ui/Button";
import {
  Card,
  CardBody,
  CardHeader,
  CardHeaderRow,
  CardHeaderTitle,
} from "~/ui/Card";
import { Dialog } from "~/ui/Dialog";
import {
  Dropdown,
  DropdownSummary,
  DropdownContent,
  DropdownButton,
} from "~/ui/Dropdown";
import { IconSpinner } from "~/ui/Icon";
import { Menu, menuChildClass } from "~/ui/Menu";
import { theme } from "~/ui/theme";
import { utility } from "~/ui/utility";

const Root = styled("div", {
  base: {
    inset: 0,
  },
});

const Viewer = styled("div", {
  base: {
    display: "flex",
    flexDirection: "column",
    gap: theme.space[4],
    padding: theme.space[4],
  },
});

const ViewerMedia = styled("div", {
  base: {
    background: theme.color.Surface0,
    border: `${theme.space.px} solid ${theme.color.Overlay0}`,
    borderRadius: theme.borderRadius,
    flex: "auto",
    height: "75vh",
  },
});

const ViewerMediaImage = styled("img", {
  base: {
    width: "100%",
    height: "100%",
    objectFit: "contain",
  },
});

const ViewerControl = styled("div", {
  base: {
    background: theme.color.Surface0,
    border: `${theme.space.px} solid ${theme.color.Overlay0}`,
    borderRadius: theme.borderRadius,
    height: theme.space[12],
    padding: theme.space[2],
  },
});

// const Debug: Component = () => {
//   return (
//     <Root>
//       <Viewer>
//         <ViewerMedia>
//           <ViewerMediaImage src="http://localhost:8090/app/stations/3b6pg5y3jjldgzl/cameras/1/fs//mnt/dvr/mmc0p2_0/2023-06-08/0/jpg/14/02/42[M][0@0][7682][3].jpg" />
//         </ViewerMedia>
//         <ViewerControl>Track</ViewerControl>
//       </Viewer>
//     </Root>
//   );
// };
//

const DialogInput = styled("input", {
  base: {
    width: "100%",
    background: theme.color.Surface0,
    fontSize: "x-large",
    padding: `${theme.space[2]}`,
    color: theme.color.Text,
    outline: "none",
    border: `${theme.space.px} solid ${theme.color.Overlay0}`,
    borderRadius: theme.borderRadius,
    ":focus": {
      borderColor: theme.color.Overlay1,
    },
  },
});

// const Debug: Component = () => {
//   return (
//     <Root>
//       <Dialog open>
//         <Stack>
//           <DialogInput placeholder="Enter command"></DialogInput>
//           <Card>
//             <CardBody>Hello</CardBody>
//           </Card>
//         </Stack>
//       </Dialog>
//     </Root>
//   );
// };
//

export const Debug: Component = () => {
  let dialog: HTMLDialogElement;
  return (
    <Root>
      <div class={style({ ...utility.stack("4"), padding: theme.space[4] })}>
        <Card>
          <CardHeader>
            <CardHeaderTitle>Card header.</CardHeaderTitle>
          </CardHeader>
          <CardBody>Card body.</CardBody>
        </Card>
        <Card>
          <CardHeader>
            <CardHeaderTitle>Card header with spinner.</CardHeaderTitle>
            <IconSpinner />
          </CardHeader>
          <CardBody>Card with header.</CardBody>
        </Card>
        <Card>
          <CardBody>Card without header.</CardBody>
        </Card>
        <Card>
          <CardBody padding={false}>
            Card body without header and padding.
          </CardBody>
        </Card>
        <Card>
          <CardHeader>
            <CardHeaderTitle>Card header only.</CardHeaderTitle>
          </CardHeader>
        </Card>
        <Card>
          <CardHeader>
            <CardHeaderTitle>Card header with empty card body.</CardHeaderTitle>
          </CardHeader>
          <CardBody></CardBody>
        </Card>
        <Card>
          <CardHeader>
            <CardHeaderTitle>
              Card header with empty card body and no padding.
            </CardHeaderTitle>
          </CardHeader>
          <CardBody padding={false}></CardBody>
        </Card>
        <Dialog ref={dialog!}>
          <Card>
            <CardHeader>Hello</CardHeader>
            <CardBody>World</CardBody>
          </Card>
        </Dialog>
        <Card>
          <CardHeader>
            <CardHeaderRow>
              <Button
                size="small"
                onClick={() => {
                  dialog.open = !dialog.open;
                }}
              >
                Primary with dropdown
              </Button>
              <Button size="small" color="danger">
                Danger
              </Button>
              <Button size="small" color="success">
                Success
              </Button>
            </CardHeaderRow>
          </CardHeader>
        </Card>
        <Dropdown>
          {() => (
            <>
              <DropdownButton>Dropdown</DropdownButton>
              <DropdownContent>
                <Menu>
                  <a href="/" class={menuChildClass}>
                    Link
                  </a>
                  <button class={menuChildClass}>Button</button>
                  <span class={menuChildClass}>Span</span>
                </Menu>
              </DropdownContent>
            </>
          )}
        </Dropdown>
        <Card>
          <CardBody>
            <div class={style({ ...utility.stack("2") })}>
              <Dropdown>
                {() => (
                  <>
                    <DropdownButton>Dropdown Button</DropdownButton>
                    <DropdownContent>
                      <Menu>
                        <a href="/" class={menuChildClass}>
                          Link
                        </a>
                        <button class={menuChildClass}>Button</button>
                        <span class={menuChildClass}>Span</span>
                      </Menu>
                    </DropdownContent>
                  </>
                )}
              </Dropdown>
              <Dropdown>
                {() => (
                  <>
                    <DropdownButton>Dropdown Button End</DropdownButton>
                    <DropdownContent end={true}>
                      <Menu>
                        <a href="/" class={menuChildClass}>
                          Link
                        </a>
                        <button class={menuChildClass}>Button</button>
                        <span class={menuChildClass}>Span</span>
                      </Menu>
                    </DropdownContent>
                  </>
                )}
              </Dropdown>
            </div>
          </CardBody>
        </Card>
        <Dropdown>
          {() => (
            <>
              <DropdownSummary>Dropdown Summary</DropdownSummary>
              <DropdownContent>
                <Menu>
                  <a href="/" class={menuChildClass}>
                    Link
                  </a>
                  <button class={menuChildClass}>Button</button>
                </Menu>
              </DropdownContent>
            </>
          )}
        </Dropdown>
        <table>
          <tbody>
            <tr>
              <td>
                <Dropdown>
                  {() => (
                    <>
                      <DropdownSummary>
                        Dropdown in a table data
                      </DropdownSummary>
                      <DropdownContent>
                        <Menu>
                          <a href="/" class={menuChildClass}>
                            Link
                          </a>
                          <button class={menuChildClass}>Button</button>
                        </Menu>
                      </DropdownContent>
                    </>
                  )}
                </Dropdown>
              </td>
              <td>Table data</td>
            </tr>
          </tbody>
        </table>
      </div>
    </Root>
  );
};
