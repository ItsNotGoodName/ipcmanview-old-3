import {
  createForm,
  FieldValues,
  FormStore,
  reset,
  ResponseData,
  SubmitHandler,
} from "@modular-forms/solid";
import {
  Accessor,
  batch,
  Component,
  createSignal,
  ParentComponent,
  Show,
} from "solid-js";

import Button from "~/ui/Button";
import { Card, CardBody, CardHeader } from "~/ui/Card";
import InputError from "~/ui/InputError";
import InputTextFrag from "~/ui/InputTextFrag";
import Spinner from "~/ui/Spinner";
import { usePb, usePbUser } from "~/data/pb";
import { PbError, UserRecord } from "~/data/records";
import { formatDateTime } from "~/data/utils";

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
  name?: string;
  username?: string;
  oldPassword?: string;
  password?: string;
  passwordConfirm?: string;
};

type UpdateFormReturn = {
  error: Accessor<string>;
  fieldErrors: Accessor<UpdateForm>;
};

const useUpdateForm = (
  form: FormStore<FieldValues, ResponseData>
): [SubmitHandler<UpdateForm>, UpdateFormReturn] => {
  const pb = usePb();
  const [{ user, updateUser }] = usePbUser();
  const [error, setError] = createSignal("");
  const [fieldErrors, setFieldErrors] = createSignal<UpdateForm>({});

  const submit: SubmitHandler<UpdateForm> = async (values) => {
    batch(() => {
      setError("");
      setFieldErrors({});
    });

    try {
      const updatedUser = await pb
        .collection("users")
        .update<UserRecord>(user().id, values);

      if (values.password) {
        // Logout on password change
        pb.authStore.clear();
      } else {
        // Update auth store
        updateUser(updatedUser);
      }

      reset(form);
    } catch (err: any) {
      const pbErr = err.data as PbError;
      let keys = Object.keys(pbErr.data) as Array<keyof UpdateForm>;
      if (keys.length > 0) {
        let newFieldErrors: UpdateForm = {};
        for (const key of keys) {
          newFieldErrors[key] = pbErr.data[key].message;
        }
        setFieldErrors(newFieldErrors);
      } else {
        setError(err.message);
      }
    }
  };

  return [submit, { error, fieldErrors }];
};

const ProfileForm: Component = () => {
  const [form, { Form, Field }] = createForm<UpdateForm, ResponseData>({});
  const [submit, { error, fieldErrors }] = useUpdateForm(form);

  return (
    <Form class="flex flex-col gap-2" onSubmit={submit} shouldDirty={true}>
      <Field name="name">
        {(field, props) => (
          <InputTextFrag
            label="New name"
            {...props}
            placeholder="New name"
            value={field.value || ""}
            error={field.error || fieldErrors()["name"]}
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
            error={field.error || fieldErrors()["username"]}
          />
        )}
      </Field>

      <Button type="submit" loading={form.submitting}>
        <div class="mx-auto">Update profile</div>
      </Button>
      <InputError error={error()} />
    </Form>
  );
};

const PasswordForm: Component = () => {
  const [form, { Form, Field }] = createForm<UpdateForm, ResponseData>();
  const [submit, { error, fieldErrors }] = useUpdateForm(form);

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
            error={field.error || fieldErrors()["oldPassword"]}
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
            error={field.error || fieldErrors()["password"]}
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
            error={field.error || fieldErrors()["passwordConfirm"]}
            autocomplete="new-password"
          />
        )}
      </Field>

      <Button type="submit" loading={form.submitting}>
        <div class="mx-auto">Update password</div>
      </Button>
      <InputError error={error()} />
    </Form>
  );
};

export default Profile;
