import theme from "styled-theming";

const primary =
  "invert(66%) sepia(58%) saturate(5354%) hue-rotate(350deg) brightness(90%) contrast(93%)";
const darkPrimary =
  "invert(32%) sepia(100%) saturate(1007%) hue-rotate(359deg) brightness(100%) contrast(93%)";
const background =
  "invert(18%) sepia(29%) saturate(447%) hue-rotate(172deg) brightness(95%) contrast(95%)";
const white =
  "invert(100%) sepia(2%) saturate(1144%) hue-rotate(326deg) brightness(115%) contrast(87%)";

export const primaryFilter = theme("theme", {
  light: darkPrimary,
  dark: primary,
});

export const backgroundFilter = theme("theme", {
  light: white,
  dark: background,
});