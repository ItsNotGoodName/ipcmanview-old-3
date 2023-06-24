import { style } from "@macaron-css/core";
import { styled } from "@macaron-css/solid";
import { createForm, reset, ResponseData } from "@modular-forms/solid";
import { Component, createSignal, Show } from "solid-js";
import { useStationApi } from "~/data/station";
import { useCreateCamera } from "~/data/station/hooks";
import { CreateCameraRequest } from "~/data/station/models";
import { createMutationForm } from "~/data/utils";
import { Button } from "~/ui/Button";
import { Card, CardBody, CardHeader, CardHeaderTitle } from "~/ui/Card";
import { Dialog } from "~/ui/Dialog";
import { ErrorText } from "~/ui/ErrorText";
import { IconSpinner } from "~/ui/Icon";
import { InputText } from "~/ui/InputText";
import { LayoutDefault } from "~/ui/Layouts";
import { utility } from "~/ui/utility";

const formClass = style({
  ...utility.stack("2"),
});

const ButtonGroup = styled("div", {
  base: {
    ...utility.row("2"),
  },
});

export const StationHome: Component = () => {
  const api = useStationApi();

  const [form, { Form, Field }] = createForm<CreateCameraRequest, ResponseData>(
    {}
  );
  const createCamera = useCreateCamera(api);
  const [formSubmit, formErrors] = createMutationForm(createCamera, form);
  const [open, setOpen] = createSignal(false);
  const toggle = () => {
    if (open()) {
      setOpen(false);
      reset(form);
      createCamera.reset();
    } else {
      setOpen(true);
    }
  };

  return (
    <LayoutDefault>
      <Dialog open={open()}>
        <Card>
          <CardHeader>
            <CardHeaderTitle>Add Camera</CardHeaderTitle>
            <Show when={form.submitting}>
              <IconSpinner />
            </Show>
          </CardHeader>
          <CardBody>
            <Form onSubmit={formSubmit} class={formClass}>
              <Field name="ip">
                {(field, props) => (
                  <InputText
                    {...props}
                    label="IP"
                    placeholder="IP"
                    value={field.value || ""}
                    error={field.error || formErrors()?.errors.ip}
                  />
                )}
              </Field>

              <Field name="username">
                {(field, props) => (
                  <InputText
                    {...props}
                    label="Username"
                    placeholder="Username"
                    value={field.value || ""}
                    error={field.error || formErrors()?.errors.username}
                  />
                )}
              </Field>

              <Field name="password">
                {(field, props) => (
                  <InputText
                    {...props}
                    type="password"
                    label="Password"
                    placeholder="Password"
                    value={field.value || ""}
                    error={field.error || formErrors()?.errors.password}
                  />
                )}
              </Field>

              <ButtonGroup>
                <Button type="submit" disabled={form.submitting}>
                  Add camera
                </Button>
                <Button type="button" onClick={toggle} color="secondary">
                  Close
                </Button>
              </ButtonGroup>

              <Show when={formErrors()}>
                {(e) => <ErrorText>{e().message}</ErrorText>}
              </Show>
            </Form>
          </CardBody>
        </Card>
      </Dialog>
      <Button onClick={toggle}>Add Camera</Button>
    </LayoutDefault>
  );
};
