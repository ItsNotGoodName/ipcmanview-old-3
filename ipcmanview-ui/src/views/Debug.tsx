import { style } from "@macaron-css/core";
import { styled } from "@macaron-css/solid";
import { Component, createSignal, JSX } from "solid-js";
import { Card, CardBody, CardHeader } from "~/ui/Card";
import Dialog from "~/ui/Dialog";
import InputText from "~/ui/InputText";
import { theme } from "~/ui/theme";
import { Row, Stack, utility } from "~/ui/utility";

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

const Debug: Component = () => {
  return (
    <Root>
      <Dialog open>
        <Stack>
          <DialogInput placeholder="Enter command"></DialogInput>
          <Card>
            <CardBody>Hello</CardBody>
          </Card>
        </Stack>
      </Dialog>
    </Root>
  );
};

export default Debug;
