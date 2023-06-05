import { ClientResponseError } from "pocketbase";
import { Component, ParentComponent, Show } from "solid-js";
import { createForm, Maybe, ResponseData } from "@modular-forms/solid";
import { createMutation } from "@tanstack/solid-query";

import Button from "~/ui/Button";
import InputError from "~/ui/InputError";
import InputText from "~/ui/InputText";
import Spinner from "~/ui/Spinner";
import { Card, CardBody, CardHeader } from "~/ui/Card";
import { UserRecord } from "~/data/records";
import { createMutationForm, formatDateTime } from "~/data/utils";
import { usePb, usePbUser } from "~/data/pb";

const Layout: ParentComponent = (props) => (
  <div class="mx-auto flex max-w-4xl flex-col gap-4 sm:flex-row">
    {props.children}
  </div>
);

const LayoutChild: ParentComponent = (props) => (
  <div class="flex flex-1 flex-col gap-4">{props.children}</div>
);

const Profile: Component = () => {
  return (
    <Layout>
      <LayoutChild>
        <div class="sticky top-0">
          <Card>
            <CardBody>
              <ProfileFrag />
            </CardBody>
          </Card>
        </div>
      </LayoutChild>
      <LayoutChild>
        <Card>
          <CardHeader>Update Profile</CardHeader>
          <CardBody>
            <ProfileForm />
          </CardBody>
        </Card>
        <Card>
          <CardHeader>Update Password</CardHeader>
          <CardBody>
            <PasswordForm />
          </CardBody>
        </Card>
      </LayoutChild>
    </Layout>
  );
};

const ProfileFrag: Component = () => {
  const [{ user }, authRefresh] = usePbUser();

  return (
    <>
      <div class="flex">
        <div class="flex-1 text-2xl">{user().username}</div>
        <Show when={authRefresh.isFetching}>
          <Spinner />
        </Show>
      </div>
      <hr class="my-2 border-base-300" />
      <table>
        <tbody>
          <tr>
            <th class="pr-2 text-right">Name</th>
            <td>{user().name}</td>
          </tr>
          <tr>
            <th class="pr-2 text-right">Username</th>
            <td>{user().username}</td>
          </tr>
          <tr>
            <th class="pr-2 text-right">Email</th>
            <td>{user().email}</td>
          </tr>
          <tr>
            <th class="pr-2 text-right">Created</th>
            <td>{formatDateTime(user().created)}</td>
          </tr>
          <tr>
            <th class="pr-2 text-right">Updated</th>
            <td>{formatDateTime(user().updated)}</td>
          </tr>
        </tbody>
      </table>
    </>
  );
};

type UpdateForm = {
  name: Maybe<string>;
  username: Maybe<string>;
  oldPassword: Maybe<string>;
  password: Maybe<string>;
  passwordConfirm: Maybe<string>;
};

const useUpdateUser = () => {
  const pb = usePb();
  const [{ user, updateUser }] = usePbUser();
  return createMutation<UserRecord, ClientResponseError, UpdateForm>({
    onSuccess: (data, variables) => {
      updateUser(data);
      if (variables.password) {
        pb.authStore.clear();
      }
    },
    mutationFn: (data: UpdateForm) =>
      pb.collection("users").update<UserRecord>(user().id, data),
  });
};

const ProfileForm: Component = () => {
  const [form, { Form, Field }] = createForm<UpdateForm, ResponseData>({});
  const [formSubmit, formErrors] = createMutationForm(useUpdateUser(), form);

  return (
    <Form class="flex flex-col gap-2" onSubmit={formSubmit} shouldDirty={true}>
      <Field name="name">
        {(field, props) => (
          <InputText
            {...props}
            label="New name"
            placeholder="New name"
            loading={form.submitting}
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
            loading={form.submitting}
            value={field.value || ""}
            error={field.error || formErrors()?.errors.username}
          />
        )}
      </Field>

      <Button type="submit" loading={form.submitting}>
        Update profile
      </Button>
      <InputError error={formErrors()?.message} />
    </Form>
  );
};

const PasswordForm: Component = () => {
  const [form, { Form, Field }] = createForm<UpdateForm, ResponseData>();
  const [submit, formErrors] = createMutationForm(useUpdateUser(), form);

  return (
    <Form class="flex flex-col gap-2" onSubmit={submit}>
      <input autocomplete="username" hidden />

      <Field name="oldPassword">
        {(field, props) => (
          <InputText
            {...props}
            label="Old password"
            type="password"
            placeholder="Old password"
            autocomplete="current-password"
            loading={form.submitting}
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
            loading={form.submitting}
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
            loading={form.submitting}
            value={field.value || ""}
            error={field.error || formErrors()?.errors.passwordConfirm}
          />
        )}
      </Field>

      <Button type="submit" loading={form.submitting}>
        Update password
      </Button>
      <InputError error={formErrors()?.message} />
    </Form>
  );
};

export default Profile;
