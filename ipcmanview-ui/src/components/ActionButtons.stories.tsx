import type { Meta, StoryObj } from "storybook-solidjs";

import "../index.css";

import ActionButtons from "./ActionButtons";

// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  component: ActionButtons,
} satisfies Meta<typeof ActionButtons>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/7.0/solid/writing-stories/args

export const Default: Story = {
  args: {
    isDeleteLoading: false,
    isRefreshLoading: false,
    isScanLoading: false,
  },
};
