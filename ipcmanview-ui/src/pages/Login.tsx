import { createForm, required, SubmitHandler } from "@modular-forms/solid";
import { Component, Show, createSignal } from "solid-js";
import pb from "../pb";
import { RiSystemLoader4Fill } from "solid-icons/ri";

type LoginForm = {
  usernameOrEmail: string;
  password: string;
};

const Home: Component = () => {
  const [form, { Form, Field }] = createForm<LoginForm>();
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
        class="mx-auto flex max-w-md flex-grow flex-col gap-2 rounded p-4 shadow"
        onSubmit={onSubmit}
      >
        <h1 class="mx-auto text-2xl">Log in</h1>

        <Field
          name="usernameOrEmail"
          validate={[required("Please enter your username or email.")]}
        >
          {(field, props) => (
            <>
              <input
                {...props}
                type="email"
                placeholder="Email or Username"
                required
                class="w-full"
              />
              <Show when={field.error}>
                <div class="text-red-500">{field.error}</div>
              </Show>
            </>
          )}
        </Field>

        <Field
          name="password"
          validate={[required("Please enter your password.")]}
        >
          {(field, props) => (
            <>
              <input
                {...props}
                type="password"
                placeholder="Password"
                class="w-full"
              />
              <Show when={field.error}>
                <div class="text-red-500">{field.error}</div>
              </Show>
            </>
          )}
        </Field>

        <button
          type="submit"
          class="flex w-full rounded bg-blue-600 p-2 text-white"
          disabled={form.submitting}
        >
          <div class="mx-auto">
            <Show
              when={!form.submitting}
              fallback={
                <div class="animate-spin">
                  <RiSystemLoader4Fill class="h-6 w-6" />
                </div>
              }
            >
              Log in
            </Show>
          </div>
        </button>
        <Show when={error}>
          <div class="text-red-500">{error()}</div>
        </Show>
      </Form>
    </div>
  );
};

export default Home;
