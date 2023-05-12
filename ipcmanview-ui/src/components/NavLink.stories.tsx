import type { Meta, StoryObj } from "storybook-solidjs";

import "../index.css";

import NavLink from "./NavLink";
import { RiBuildingsHome5Line } from "solid-icons/ri";
import { Router } from "@solidjs/router";

// More on how to set up stories at: https://storybook.js.org/docs/7.0/solid/writing-stories/introduction
const meta = {
  component: NavLink,
  decorators: [
    (Story) => (
      <Router>
        <Story />
      </Router>
    ),
  ],
} satisfies Meta<typeof NavLink>;

export default meta;
type Story = StoryObj<typeof meta>;

// More on writing stories with args: https://storybook.js.org/docs/7.0/solid/writing-stories/args

export const Home: Story = {
  args: {
    href: "/",
    title: "Home",
    children: <RiBuildingsHome5Line class="h-6 w-6" />,
  },
};
