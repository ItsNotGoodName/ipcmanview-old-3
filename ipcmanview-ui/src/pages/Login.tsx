import { createForm, required, SubmitHandler } from "@modular-forms/solid";
import { Component, createSignal, ParentComponent } from "solid-js";

import Button from "../components/Button";
import { Card, CardBody, CardHeader } from "../components/Card";
import InputError from "../components/InputError";
import InputTextFrag from "../components/InputTextFrag";
import { usePb } from "../pb";
import { ADMIN_PANEL_URL } from "../utils";

type LoginForm = {
  usernameOrEmail: string;
  password: string;
};

const CenterLayout: ParentComponent = (props) => (
  <div class="px-4 py-16">
    <div class="mx-auto flex max-w-sm flex-col gap-4">{props.children}</div>
  </div>
);

const Home: Component = () => {
  const [form, { Form, Field }] = createForm<LoginForm>({});
  const [error, setError] = createSignal("");

  const pb = usePb();
  const onSubmit: SubmitHandler<LoginForm> = (values) =>
    pb
      .collection("users")
      .authWithPassword(values.usernameOrEmail, values.password)
      .catch((err) => {
        setError(err.message);
      });

  return (
    <CenterLayout>
      <Card>
        <CardHeader title={<div class="text-center">IPCManView</div>} />
        <CardBody>
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
              href={ADMIN_PANEL_URL}
            >
              Forgot Password?
            </a>

            <Button type="submit" loading={form.submitting}>
              <div class="mx-auto">Log in</div>
            </Button>
            <InputError error={error()} />
          </Form>
        </CardBody>
      </Card>
      <a
        class="ml-auto text-sm text-link hover:underline"
        href={ADMIN_PANEL_URL}
      >
        Admin Panel
      </a>
    </CenterLayout>
  );
};

export default Home;
