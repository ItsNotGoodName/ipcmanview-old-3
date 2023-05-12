import type { Meta, StoryObj } from "storybook-solidjs";

import "../index.css";

import FormTextInput from "./FormTextInput";

// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  component: FormTextInput,
} satisfies Meta<typeof FormTextInput>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/7.0/solid/writing-stories/args

export const Default: Story = {
  args: {
    loading: false,
    error: "",
    placeholder: "",
    required: false,
    label: "",
  },
};
