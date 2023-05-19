import type { Meta, StoryObj } from "storybook-solidjs";

import "../index.css";

import Spinner from "./Spinner";

// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  component: Spinner,
} satisfies Meta<typeof Spinner>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/7.0/solid/writing-stories/args

export const Default: Story = {};

export const Center: Story = {
  decorators: [
    (Story) => (
      <div class="flex gap-2">
        <div class="text-4xl">Hello World</div>
        <div class="my-auto">
          <Story />
        </div>
      </div>
    ),
  ],
};
