import type { Meta, StoryObj } from "storybook-solidjs";

import "../index.css";

import Header from "./Header";
import { Router } from "@solidjs/router";

// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  component: Header,
  decorators: [
    (Story) => (
      <Router>
        <Story />
      </Router>
    ),
  ],
} satisfies Meta<typeof Header>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/7.0/solid/writing-stories/args

export const Default: Story = {};
