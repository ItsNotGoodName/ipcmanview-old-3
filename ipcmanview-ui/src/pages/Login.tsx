import { createForm, required, SubmitHandler } from "@modular-forms/solid";
import { Component, Show, createSignal } from "solid-js";
import pb from "../pb";
import Button from "../components/Button";
import FormTextInput from "../components/FormTextInput";
import InputError from "../components/InputError";

type LoginForm = {
  usernameOrEmail: string;
  password: string;
};

const Home: Component = () => {
  const [form, { Form, Field }] = createForm<LoginForm>({});
  const [error, setError] = createSignal("");

  const onSubmit: SubmitHandler<LoginForm> = (values) =>
    pb
      .collection("users")
      .authWithPassword(values.usernameOrEmail, values.password)
      .catch((err) => {
        setError(err.message);
      });

  return (
    <div class="flex items-center px-4 py-16">
      <Form
        class="mx-auto flex max-w-sm flex-grow flex-col gap-2 rounded p-4 shadow shadow-ship-300"
        onSubmit={onSubmit}
      >
        <h1 class="mx-auto text-2xl">Log in</h1>

        <Field
          name="usernameOrEmail"
          validate={[required("Please enter your username or email.")]}
        >
          {(field, props) => (
            <FormTextInput
              {...props}
              label="Username or Email"
              placeholder="Username or Email"
              autocomplete="username"
              error={field.error}
            />
          )}
        </Field>

        <Field
          name="password"
          validate={[required("Please enter your password.")]}
        >
          {(field, props) => (
            <FormTextInput
              {...props}
              label="Password"
              type="password"
              placeholder="Password"
              autocomplete="current-password"
              error={field.error}
            />
          )}
        </Field>

        <Button class="mt-2" type="submit" loading={form.submitting}>
          <div class="mx-auto">Log in</div>
        </Button>
        <InputError error={error()} />
      </Form>
    </div>
  );
};

export default Home;
