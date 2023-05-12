import { createForm, email, SubmitHandler } from "@modular-forms/solid";
import clsx from "clsx";
import { Component, createSignal, Show } from "solid-js";
import Button from "../components/Button";
import FormTextInput from "../components/FormTextInput";
import InputError from "../components/InputError";
import pb, { authStore, eagerUpdateUser, UserRecord } from "../pb";
import { formatDateTime } from "../utils";

const Profile: Component = () => {
  return (
    <div class="mx-auto flex max-w-4xl flex-wrap gap-4">
      <div class="flex-1">
        <div class="rounded p-4 shadow shadow-ship-300">
          <h1 class="text-2xl">{authStore().model?.username}</h1>
          <hr class="my-2 text-ship-300" />
          <table class="table">
            <tbody>
              <tr>
                <th class="pr-2 text-right">Name</th>
                <td>{authStore().model?.name}</td>
              </tr>
              <tr>
                <th class="pr-2 text-right">Username</th>
                <td>{authStore().model?.username}</td>
              </tr>
              <tr>
                <th class="pr-2 text-right">Email</th>
                <td>{authStore().model?.email}</td>
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
        </div>
      </div>
      <UpdateForm class="w-full flex-1 rounded shadow shadow-ship-300 sm:max-w-sm" />
    </div>
  );
};

type UpdateForm = {
  name?: string;
  username?: string;
  email?: string;
};

const UpdateForm: Component<{ class?: string }> = (props) => {
  const [form, { Form, Field }] = createForm<UpdateForm>({});
  const [error, setError] = createSignal("");

  const onSubmit: SubmitHandler<UpdateForm> = (values) => {
    setError("");

    // TODO: make the form send undefined
    for (let key in values) {
      //@ts-ignore
      if (values[key] == "") {
        //@ts-ignore
        values[key] = undefined;
      }
    }

    return pb
      .collection("users")
      .update<UserRecord>(authStore().model!.id, values)
      .then((user) => eagerUpdateUser(user))
      .catch((err) => {
        setError(err.message);
      });
  };

  return (
    <Form
      class={clsx(
        "flex flex-col gap-2 rounded p-4 shadow-ship-300",
        props.class
      )}
      onSubmit={onSubmit}
    >
      <Field name="name">
        {(field, props) => (
          <FormTextInput
            label="Name"
            {...props}
            placeholder="Name"
            error={field.error}
          />
        )}
      </Field>

      <Field name="username">
        {(field, props) => (
          <FormTextInput
            label="Username"
            {...props}
            placeholder="Username"
            error={field.error}
          />
        )}
      </Field>

      <Field name="email" validate={[email("Please enter a valid email.")]}>
        {(field, props) => (
          <FormTextInput
            label="Email"
            {...props}
            placeholder="Email"
            error={field.error}
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

export default Profile;
