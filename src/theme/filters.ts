import theme from "styled-theming";

const primary = "invert(66%) sepia(58%) saturate(5354%) hue-rotate(350deg) brightness(90%) contrast(93%);"
const darkPrimary = "invert(32%) sepia(100%) saturate(1007%) hue-rotate(359deg) brightness(100%) contrast(93%);"

export const primaryFilter = theme("theme", {
	light: darkPrimary,
	dark: primary,
});