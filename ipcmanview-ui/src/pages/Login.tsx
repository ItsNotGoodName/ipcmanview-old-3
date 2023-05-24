import { createForm, required, SubmitHandler } from "@modular-forms/solid";
import { RiSystemQuestionFill } from "solid-icons/ri";
import { Component, createSignal } from "solid-js";

import Button from "../components/Button";
import Card from "../components/Card";
import InputError from "../components/InputError";
import InputTextFrag from "../components/InputTextFrag";
import pb, { adminPageUrl } from "../pb";

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
    <div class="px-4 py-16">
      <div class="mx-auto flex max-w-sm flex-col gap-4">
        <Card.HeaderCard title={<div class="text-center">IPCManView</div>}>
          <Card.Body>
            <Form class="flex flex-col gap-2" onSubmit={onSubmit}>
              <Field
                name="usernameOrEmail"
                validate={[required("Please enter your username or email.")]}
              >
                {(field, props) => (
                  <InputTextFrag
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
                  <InputTextFrag
                    {...props}
                    label="Password"
                    type="password"
                    placeholder="Password"
                    autocomplete="current-password"
                    error={field.error}
                  />
                )}
              </Field>

              <a
                class="ml-auto text-sm text-link hover:underline"
                href={adminPageUrl}
              >
                Forgot Password?
              </a>

              <Button class="mt-2" type="submit" loading={form.submitting}>
                <div class="mx-auto">Log in</div>
              </Button>
              <InputError error={error()} />
            </Form>
          </Card.Body>
        </Card.HeaderCard>
        <a
          class="ml-auto text-sm text-link hover:underline"
          href={adminPageUrl}
        >
          Admin Panel
        </a>
      </div>
    </div>
  );
};

export default Home;
