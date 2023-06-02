import { createForm, Maybe, ResponseData } from "@modular-forms/solid";
import { Component, ParentComponent, Show } from "solid-js";
import { createMutation } from "@tanstack/solid-query";
import { ClientResponseError } from "pocketbase";

import Button from "~/ui/Button";
import { Card, CardBody, CardHeader } from "~/ui/Card";
import InputError from "~/ui/InputError";
import InputTextFrag from "~/ui/InputTextFrag";
import Spinner from "~/ui/Spinner";
import { usePb, usePbUser } from "~/data/pb";
import { UserRecord } from "~/data/records";
import { createMutationForm, formatDateTime } from "~/data/utils";

const DualLayout: ParentComponent = (props) => (
  <div class="mx-auto flex max-w-4xl flex-col gap-4 sm:flex-row">
    {props.children}
  </div>
);

const DualLayoutChild: ParentComponent = (props) => (
  <div class="flex flex-1 flex-col gap-4">{props.children}</div>
);

const Profile: Component = () => {
  return (
    <DualLayout>
      <DualLayoutChild>
        <div class="sticky top-0">
          <Card>
            <CardBody>
              <ProfileFrag />
            </CardBody>
          </Card>
        </div>
      </DualLayoutChild>
      <DualLayoutChild>
        <Card>
          <CardHeader title="Update Profile" />
          <CardBody>
            <ProfileForm />
          </CardBody>
        </Card>
        <Card>
          <CardHeader title="Update Password" />
          <CardBody>
            <PasswordForm />
          </CardBody>
        </Card>
      </DualLayoutChild>
    </DualLayout>
  );
};

const ProfileFrag: Component = () => {
  const [{ user }, authRefresh] = usePbUser();

  return (
    <>
      <div class="flex">
        <h1 class="flex-1 text-2xl">{user().username}</h1>
        <Show when={authRefresh.isFetching}>
          <Spinner />
        </Show>
      </div>
      <hr class="my-2 text-ship-600" />
      <table class="table">
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
          <InputTextFrag
            label="New name"
            {...props}
            placeholder="New name"
            value={field.value || ""}
            error={field.error || formErrors()?.errors.name}
          />
        )}
      </Field>

      <Field name="username">
        {(field, props) => (
          <InputTextFrag
            label="New username"
            {...props}
            placeholder="New username"
            value={field.value || ""}
            error={field.error || formErrors()?.errors.username}
          />
        )}
      </Field>

      <Button type="submit" loading={form.submitting}>
        <div class="mx-auto">Update profile</div>
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
          <InputTextFrag
            label="Old password"
            type="password"
            {...props}
            placeholder="Old password"
            value={field.value || ""}
            error={field.error || formErrors()?.errors.oldPassword}
            autocomplete="current-password"
          />
        )}
      </Field>

      <Field name="password">
        {(field, props) => (
          <InputTextFrag
            label="New Password"
            type="password"
            {...props}
            placeholder="New password"
            value={field.value || ""}
            error={field.error || formErrors()?.errors.password}
            autocomplete="new-password"
          />
        )}
      </Field>

      <Field name="passwordConfirm">
        {(field, props) => (
          <InputTextFrag
            label="Confirm new password"
            type="password"
            {...props}
            placeholder="Confirm new password"
            value={field.value || ""}
            error={field.error || formErrors()?.errors.passwordConfirm}
            autocomplete="new-password"
          />
        )}
      </Field>

      <Button type="submit" loading={form.submitting}>
        <div class="mx-auto">Update password</div>
      </Button>
      <InputError error={formErrors()?.message} />
    </Form>
  );
};

export default Profile;
