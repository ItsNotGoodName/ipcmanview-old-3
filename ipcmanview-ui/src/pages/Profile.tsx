import { ClientResponseError } from "pocketbase";
import { Component, Show } from "solid-js";
import { createForm, Maybe, reset, ResponseData } from "@modular-forms/solid";
import { createMutation } from "@tanstack/solid-query";
import { styled } from "@macaron-css/solid";
import { style } from "@macaron-css/core";

import { Button } from "~/ui/Button";
import { ErrorText } from "~/ui/ErrorText";
import { InputText } from "~/ui/InputText";
import { Card, CardBody, CardHeader, CardHeaderTitle } from "~/ui/Card";
import { UserRecord } from "~/data/pb/records";
import { createMutationForm, formatDateTime } from "~/data/utils";
import { usePb, usePbUser } from "~/data/pb";
import { minScreen, theme } from "~/ui/theme";
import { IconSpinner } from "~/ui/Icon";
import { utility } from "~/ui/utility";

const Layout = styled("div", {
  base: {
    padding: theme.space[4],
    margin: "0 auto 0 auto",
    display: "flex",
    gap: theme.space[4],
    flexDirection: "column",
    maxWidth: theme.size["lg"],
    "@media": {
      [minScreen.md]: {
        flexDirection: "row",
      },
    },
  },
});

const LayoutChild = styled("div", {
  base: {
    display: "flex",
    flex: "1",
    gap: theme.space[4],
    flexDirection: "column",
  },
});

const Sticky = styled("div", {
  base: {
    top: "1",
    position: "sticky",
  },
});

export const Profile: Component = () => {
  return (
    <Layout>
      <LayoutChild>
        <Sticky>
          <ProfileCard />
        </Sticky>
      </LayoutChild>
      <LayoutChild>
        <ProfileFormCard />
        <PasswordFormCard />
      </LayoutChild>
    </Layout>
  );
};

const ProfileFragTitle = styled("div", {
  base: {
    fontSize: "x-large",
    paddingBottom: theme.space[2],
    marginBottom: theme.space[2],
    borderBottom: `${theme.space.px} solid ${theme.color.Overlay0}`,
  },
});

const Th = styled("th", {
  base: {
    textAlign: "right",
    paddingRight: theme.space[2],
  },
});

const ProfileCard: Component = () => {
  const { user } = usePbUser();

  return (
    <Card>
      <CardBody>
        <ProfileFragTitle>{user().username}</ProfileFragTitle>
        <table>
          <tbody>
            <tr>
              <Th>Name</Th>
              <td>{user().name}</td>
            </tr>
            <tr>
              <Th>Username</Th>
              <td>{user().username}</td>
            </tr>
            <tr>
              <Th>Email</Th>
              <td>{user().email}</td>
            </tr>
            <tr>
              <Th>Created</Th>
              <td>{formatDateTime(user().created)}</td>
            </tr>
            <tr>
              <Th>Updated</Th>
              <td>{formatDateTime(user().updated)}</td>
            </tr>
          </tbody>
        </table>
      </CardBody>
    </Card>
  );
};

const formClass = style({
  ...utility.stack("4"),
});

type UpdateForm = {
  name: Maybe<string>;
  username: Maybe<string>;
  oldPassword: Maybe<string>;
  password: Maybe<string>;
  passwordConfirm: Maybe<string>;
};

const useUpdateUser = () => {
  const pb = usePb();
  const { user, set: setUser } = usePbUser();

  return createMutation<UserRecord, ClientResponseError, UpdateForm>({
    onSuccess: (data, variables) => {
      setUser(data);
      if (variables.password) {
        pb.authStore.clear();
      }
    },
    mutationFn: (data: UpdateForm) =>
      pb.collection("users").update<UserRecord>(user().id, data),
  });
};

const ProfileFormCard: Component = () => {
  const [form, { Form, Field }] = createForm<UpdateForm, ResponseData>({});
  const [formSubmit, formErrors] = createMutationForm(useUpdateUser(), {
    onSuccess() {
      reset(form);
    },
  });

  return (
    <Card>
      <CardHeader>
        <CardHeaderTitle>Update Profile</CardHeaderTitle>
        <Show when={form.submitting}>
          <IconSpinner />
        </Show>
      </CardHeader>
      <CardBody>
        <Form onSubmit={formSubmit} shouldDirty={true} class={formClass}>
          <Field name="name">
            {(field, props) => (
              <InputText
                {...props}
                label="New name"
                placeholder="New name"
                value={field.value || ""}
                error={field.error || formErrors()?.errors.name}
              />
            )}
          </Field>

          <Field name="username">
            {(field, props) => (
              <InputText
                {...props}
                label="New username"
                placeholder="New username"
                value={field.value || ""}
                error={field.error || formErrors()?.errors.username}
              />
            )}
          </Field>

          <Button type="submit" disabled={form.submitting}>
            Update profile
          </Button>
          <Show when={formErrors()}>
            {(e) => <ErrorText>{e().message}</ErrorText>}
          </Show>
        </Form>
      </CardBody>
    </Card>
  );
};

const PasswordFormCard: Component = () => {
  const [form, { Form, Field }] = createForm<UpdateForm, ResponseData>();
  const [submit, formErrors] = createMutationForm(useUpdateUser(), {
    onSuccess() {
      reset(form);
    },
  });

  return (
    <Card>
      <CardHeader>
        <CardHeaderTitle>Update Password</CardHeaderTitle>
        <Show when={form.submitting}>
          <IconSpinner />
        </Show>
      </CardHeader>
      <CardBody>
        <Form onSubmit={submit} class={formClass}>
          <input autocomplete="username" hidden />

          <Field name="oldPassword">
            {(field, props) => (
              <InputText
                {...props}
                label="Old password"
                type="password"
                placeholder="Old password"
                autocomplete="current-password"
                value={field.value || ""}
                error={field.error || formErrors()?.errors.oldPassword}
              />
            )}
          </Field>

          <Field name="password">
            {(field, props) => (
              <InputText
                {...props}
                label="New Password"
                type="password"
                placeholder="New password"
                autocomplete="new-password"
                value={field.value || ""}
                error={field.error || formErrors()?.errors.password}
              />
            )}
          </Field>

          <Field name="passwordConfirm">
            {(field, props) => (
              <InputText
                {...props}
                label="Confirm new password"
                type="password"
                placeholder="Confirm new password"
                autocomplete="new-password"
                value={field.value || ""}
                error={field.error || formErrors()?.errors.passwordConfirm}
              />
            )}
          </Field>

          <Button type="submit" disabled={form.submitting}>
            Update password
          </Button>
          <Show when={formErrors()}>
            {(e) => <ErrorText>{e().message}</ErrorText>}
          </Show>
        </Form>
      </CardBody>
    </Card>
  );
};
