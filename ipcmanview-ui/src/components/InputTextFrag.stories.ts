import type { Meta, StoryObj } from "storybook-solidjs";

import "../index.css";

import InputTextFrag from "./InputTextFrag";

// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  component: InputTextFrag,
} satisfies Meta<typeof InputTextFrag>;

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
