import theme from "styled-theming";
import { primaryFilter } from "../../theme/filters";

const filters = [
	theme("theme", {
		light: "invert(34%) sepia(9%) saturate(978%) hue-rotate(169deg) brightness(99%) contrast(91%)", 
		dark: "invert(79%) sepia(6%) saturate(544%) hue-rotate(171deg) brightness(91%) contrast(90%)"
	}),
	theme("theme", {
		light: "invert(33%) sepia(39%) saturate(987%) hue-rotate(232deg) brightness(99%) contrast(91%)", 
		dark: "invert(75%) sepia(23%) saturate(2145%) hue-rotate(231deg) brightness(91%) contrast(91%)"
	}),
	theme("theme", {
		light: "invert(33%) sepia(69%) saturate(997%) hue-rotate(295deg) brightness(100%) contrast(92%)", 
		dark: "invert(70%) sepia(41%) saturate(3746%) hue-rotate(290deg) brightness(90%) contrast(92%)"
	}),
	primaryFilter
];

export default filters;