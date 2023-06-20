import PocketBase, { ClientResponseError } from "pocketbase";
import { Component, Show } from "solid-js";
import { createForm, required, ResponseData } from "@modular-forms/solid";
import { createMutation } from "@tanstack/solid-query";
import { styled } from "@macaron-css/solid";
import { style } from "@macaron-css/core";

import { Button } from "~/ui/Button";
import { ErrorText } from "~/ui/ErrorText";
import { InputText } from "~/ui/InputText";
import { ADMIN_PANEL_URL, createMutationForm } from "~/data/utils";
import { Card, CardBody, CardHeader, CardHeaderTitle } from "~/ui/Card";
import { LayoutCenter } from "~/ui/Layouts";
import { usePb } from "~/data/pb";
import { ThemeSwitcher, ThemeSwitcherIcon } from "~/ui/ThemeSwitcher";
import { theme } from "~/ui/theme";
import { utility } from "~/ui/utility";

const Center = styled("div", {
  base: {
    display: "flex",
    justifyContent: "center",
  },
});

const Right = styled("div", {
  base: {
    display: "flex",
    justifyContent: "end",
  },
});

const Title = styled("div", {
  base: {
    ...utility.textLine(),
  },
});

const Stack = styled("div", {
  base: {
    ...utility.stack("4"),
  },
});

const themeSwitcherClass = style({
  display: "flex",
  alignItems: "center",
  borderRadius: theme.borderRadius,
  ":hover": {
    backgroundColor: theme.color.Surface2,
  },
});

type LoginMutation = {
  usernameOrEmail: string;
  password: string;
};

const useLoginMutation = (pb: PocketBase) =>
  createMutation<unknown, ClientResponseError, LoginMutation>({
    mutationFn: (data: LoginMutation) =>
      pb
        .collection("users")
        .authWithPassword(data.usernameOrEmail, data.password),
  });

export const Login: Component = () => {
  const [form, { Form, Field }] = createForm<LoginMutation, ResponseData>({});
  const [formSubmit, formErrors] = createMutationForm(
    useLoginMutation(usePb())
  );

  return (
    <LayoutCenter>
      <Card>
        <CardHeader>
          <CardHeaderTitle>IPCManView</CardHeaderTitle>
          <ThemeSwitcher class={themeSwitcherClass}>
            <ThemeSwitcherIcon class={style({ ...utility.size("6") })} />
          </ThemeSwitcher>
        </CardHeader>
        <CardBody>
          <Form onSubmit={formSubmit}>
            <Stack>
              <Field
                name="usernameOrEmail"
                validate={[required("Please enter your username or email.")]}
              >
                {(field, props) => (
                  <InputText
                    {...props}
                    label="Username or Email"
                    placeholder="Username or Email"
                    autocomplete="username"
                    disabled={form.submitting}
                    error={field.error}
                  />
                )}
              </Field>

              <Field
                name="password"
                validate={[required("Please enter your password.")]}
              >
                {(field, props) => (
                  <InputText
                    {...props}
                    label="Password"
                    type="password"
                    placeholder="Password"
                    autocomplete="current-password"
                    disabled={form.submitting}
                    error={field.error}
                  />
                )}
              </Field>

              <Right>
                <a href={ADMIN_PANEL_URL}>Forgot Password?</a>
              </Right>

              <Button type="submit" disabled={form.submitting}>
                Log in
              </Button>
              <Show when={formErrors()}>
                {(e) => <ErrorText>{e().message}</ErrorText>}
              </Show>
            </Stack>
          </Form>
        </CardBody>
      </Card>
      <Center>
        <a href={ADMIN_PANEL_URL}>Admin Panel</a>
      </Center>
    </LayoutCenter>
  );
};
