/** @type {import('tailwindcss').Config} */
module.exports = {
  addons: [
    "@storybook/addon-links",
    "@storybook/addon-essentials",
    "@storybook/addon-interactions",
  ],
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {},
    colors: {
      ship: {
        50: "#f4f7fa",
        100: "#e7ecf2",
        200: "#d4dee9",
        300: "#b6c8da",
        400: "#93abc7",
        500: "#839bbe",
        600: "#667daa",
        700: "#5b6d9a",
        800: "#4e5a7f",
        900: "#424c66",
        950: "#2b3140",
      },
      danger: "#ef4444",
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
