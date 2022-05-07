import theme from "styled-theming";

const primary = "#df691a";
const darkPrimary = "#d05a0b";

const background = "#2a3542";

const gray = "#abb6c2";
const darkGray = "#4e5d6c";

const white = "#eee";

export const backgroundColor = theme("theme", {
  light: white,
  dark: background,
});

export const primaryColor = theme("theme", {
  light: darkPrimary,
  dark: primary,
});

export const frontColor = theme("theme", {
  light: darkGray,
  dark: gray,
});

export const textColor = theme("theme", {
  light: background,
  dark: white,
});

export const hoverPrimaryColor = theme("theme", {
  light: "#c04a00",
  dark: "#f57e2f",
});
