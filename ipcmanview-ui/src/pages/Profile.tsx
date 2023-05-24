import {
  createForm,
  FieldValues,
  FormStore,
  reset,
  ResponseData,
  SubmitHandler,
} from "@modular-forms/solid";
import clsx from "clsx";
import { Accessor, batch, Component, createSignal, Show } from "solid-js";

import Button from "../components/Button";
import Card from "../components/Card";
import InputError from "../components/InputError";
import InputTextFrag from "../components/InputTextFrag";
import Spinner from "../components/Spinner";
import pb, { authStore, authStoreMutate } from "../pb";
import { PbError, UserRecord } from "../records";
import { formatDateTime } from "../utils";
import { useAuthRefresh } from "../hooks";

const Profile: Component = () => {
  return (
    <div class="mx-auto flex max-w-4xl flex-col gap-4 sm:flex-row">
      <div class="flex flex-1 flex-col gap-4">
        <Card.NormalCard class="sticky top-0">
          <Card.Body>
            <ProfileFrag />
          </Card.Body>
        </Card.NormalCard>
      </div>
      <div class="flex flex-1 flex-col gap-4 rounded sm:max-w-sm">
        <Card.HeaderCard title="Update Profile">
          <Card.Body>
            <ProfileForm />
          </Card.Body>
        </Card.HeaderCard>
        <Card.HeaderCard title="Update Password">
          <Card.Body>
            <PasswordForm />
          </Card.Body>
        </Card.HeaderCard>
      </div>
    </div>
  );
};

const ProfileFrag: Component = () => {
  const authRefresh = useAuthRefresh(true);

  return (
    <>
      <div class="flex">
        <h1 class="flex-1 text-2xl">{authStore().model!.username}</h1>
        <Show when={authRefresh.isFetching}>
          <Spinner />
        </Show>
      </div>
      <hr class="my-2 text-ship-300" />
      <table class="table">
        <tbody>
          <tr>
            <th class="pr-2 text-right">Name</th>
            <td>{authStore().model!.name}</td>
          </tr>
          <tr>
            <th class="pr-2 text-right">Username</th>
            <td>{authStore().model!.username}</td>
          </tr>
          <tr>
            <th class="pr-2 text-right">Email</th>
            <td>{authStore().model!.email}</td>
          </tr>
          <tr>
            <th class="pr-2 text-right">Created</th>
            <td>{formatDateTime(authStore().model!.created)}</td>
          </tr>
          <tr>
            <th class="pr-2 text-right">Updated</th>
            <td>{formatDateTime(authStore().model!.updated)}</td>
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
  const [error, setError] = createSignal("");
  const [fieldErrors, setFieldErrors] = createSignal<UpdateForm>({});

  const submit: SubmitHandler<UpdateForm> = async (values) => {
    batch(() => {
      setError("");
      setFieldErrors({});
    });

    try {
      const user = await pb
        .collection("users")
        .update<UserRecord>(authStore().model!.id, values);

      if (values.password) {
        // Logout on password change
        pb.authStore.clear();
      } else {
        // Update auth store
        authStoreMutate(user);
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

const ProfileForm: Component<{ class?: string }> = (props) => {
  const [form, { Form, Field }] = createForm<UpdateForm, ResponseData>({});
  const [submit, { error, fieldErrors }] = useUpdateForm(form);

  return (
    <Form
      class={clsx("flex flex-col gap-2", props.class)}
      onSubmit={submit}
      shouldDirty={true}
    >
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

      <Button class="mt-2" type="submit" loading={form.submitting}>
        <div class="mx-auto">Update profile</div>
      </Button>
      <InputError error={error()} />
    </Form>
  );
};

const PasswordForm: Component<{ class?: string }> = (props) => {
  const [form, { Form, Field }] = createForm<UpdateForm, ResponseData>();
  const [submit, { error, fieldErrors }] = useUpdateForm(form);

  return (
    <Form class={clsx("flex flex-col gap-2", props.class)} onSubmit={submit}>
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

      <Button class="mt-2" type="submit" loading={form.submitting}>
        <div class="mx-auto">Update password</div>
      </Button>
      <InputError error={error()} />
    </Form>
  );
};

export default Profile;
