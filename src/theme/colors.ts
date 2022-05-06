import theme from "styled-theming";

const primary = "#df691a";
const darkPrimary = "#d05a0b";
const secondary = "#2a3542";

//const secondary = "#4e5d6c";
//const light = "#abb6c2";

const white = "#eee";

export const backgroundColor = theme("theme", {
  light: white,
  dark: secondary,
});

export const primaryColor = theme("theme", {
  light: darkPrimary,
  dark: primary,
});

export const textColor = theme("theme", {
  light: secondary,
  dark: white,
});

export const hoverPrimaryColor = theme("theme", {
  light: "#c04a00",
  dark: "#f57e2f",
});
