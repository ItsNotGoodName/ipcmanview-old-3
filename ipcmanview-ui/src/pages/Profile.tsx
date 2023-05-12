import {
  createForm,
  FieldValues,
  FormStore,
  reset,
  ResponseData,
  SubmitHandler,
} from "@modular-forms/solid";
import clsx from "clsx";
import { Accessor, batch, Component, createSignal } from "solid-js";
import Button from "../components/Button";
import InputText from "../components/InputText";
import InputError from "../components/InputError";
import pb, { authStore, eagerUpdateUser, PbError, UserRecord } from "../pb";
import { formatDateTime } from "../utils";

const Profile: Component = () => {
  return (
    <div class="mx-auto flex max-w-4xl flex-wrap gap-4">
      <div class="flex flex-1 flex-col gap-4">
        <div class="rounded p-4 shadow shadow-ship-300">
          <ProfileFrag />
        </div>
      </div>
      <div class="flex flex-1 flex-col gap-4 rounded sm:max-w-sm">
        <ProfileForm class="rounded p-4 shadow shadow-ship-300" />
        <PasswordForm class="rounded p-4 shadow shadow-ship-300" />
      </div>
    </div>
  );
};

const ProfileFrag: Component = () => {
  return (
    <>
      <h1 class="text-2xl">{authStore().model?.username}</h1>
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

  const submit: SubmitHandler<UpdateForm> = (values) => {
    batch(() => {
      setError("");
      setFieldErrors({});
    });

    return pb
      .collection("users")
      .update<UserRecord>(authStore().model!.id, values)
      .then((user) => {
        if (values.password) {
          pb.authStore.clear();
        } else {
          eagerUpdateUser(user);
        }
        reset(form);
      })
      .catch((err) => {
        const pbErr = err.data as PbError;
        let keys = Object.keys(pbErr.data);
        if (keys.length < 0) {
          setError(err.message);
          return;
        }

        let newFieldErrors = {};
        for (const key of keys) {
          //@ts-ignore
          newFieldErrors[key] = pbErr.data[key].message;
        }
        setFieldErrors(newFieldErrors);
      });
  };

  return [submit, { error, fieldErrors }];
};

const ProfileForm: Component<{ class?: string }> = (props) => {
  const [form, { Form, Field }] = createForm<UpdateForm>({});
  const [submit, { error, fieldErrors }] = useUpdateForm(form);

  return (
    <Form
      class={clsx("flex flex-col gap-2", props.class)}
      onSubmit={submit}
      shouldDirty={true}
    >
      <Field name="name">
        {(field, props) => (
          <InputText
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
          <InputText
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
  const [form, { Form, Field }] = createForm<UpdateForm>();
  const [submit, { error, fieldErrors }] = useUpdateForm(form);

  return (
    <Form class={clsx("flex flex-col gap-2", props.class)} onSubmit={submit}>
      <input autocomplete="username" hidden />

      <Field name="oldPassword">
        {(field, props) => (
          <InputText
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
          <InputText
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
          <InputText
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
