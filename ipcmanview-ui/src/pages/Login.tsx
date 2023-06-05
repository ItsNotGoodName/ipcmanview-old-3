import PocketBase, { ClientResponseError } from "pocketbase";
import { Component } from "solid-js";
import { createForm, required, ResponseData } from "@modular-forms/solid";
import { createMutation } from "@tanstack/solid-query";

import Button from "~/ui/Button";
import InputError from "~/ui/InputError";
import InputText from "~/ui/InputText";
import { ADMIN_PANEL_URL, createMutationForm } from "~/data/utils";
import { Card, CardBody, CardHeader } from "~/ui/Card";
import { CenterLayout } from "~/ui/Layouts";
import { usePb } from "~/data/pb";
import ThemeSwitcher from "~/ui/ThemeSwitcher";

type LoginMutation = {
  usernameOrEmail: string;
  password: string;
};

const useLoginMutation = (pb: PocketBase) =>
  createMutation<unknown, ClientResponseError, LoginMutation>({
    mutationFn: (data: LoginMutation) =>
      pb
        .collection("users")
        .authWithPassword(data.usernameOrEmail, data.password),
  });

const Login: Component = () => {
  const [form, { Form, Field }] = createForm<LoginMutation, ResponseData>({});
  const [formSubmit, formErrors] = createMutationForm(
    useLoginMutation(usePb()),
    form
  );

  return (
    <CenterLayout>
      <Card>
        <CardHeader right={<ThemeSwitcher />}>Login</CardHeader>
        <CardBody>
          <Form class="flex flex-col gap-2" onSubmit={formSubmit}>
            <Field
              name="usernameOrEmail"
              validate={[required("Please enter your username or email.")]}
            >
              {(field, props) => (
                <InputText
                  {...props}
                  label="Username or Email"
                  placeholder="Username or Email"
                  autocomplete="username"
                  loading={form.submitting}
                  error={field.error}
                />
              )}
            </Field>

            <Field
              name="password"
              validate={[required("Please enter your password.")]}
            >
              {(field, props) => (
                <InputText
                  {...props}
                  label="Password"
                  type="password"
                  placeholder="Password"
                  autocomplete="current-password"
                  loading={form.submitting}
                  error={field.error}
                />
              )}
            </Field>

            <a
              class="link-hover link-info link ml-auto text-sm"
              href={ADMIN_PANEL_URL}
            >
              Forgot Password?
            </a>

            <Button type="submit" loading={form.submitting}>
              Log in
            </Button>
            <InputError error={formErrors()?.message} />
          </Form>
        </CardBody>
      </Card>
      <a
        class="link-hover link-info link mx-auto text-sm"
        href={ADMIN_PANEL_URL}
      >
        Admin Panel
      </a>
    </CenterLayout>
  );
};

export default Login;
