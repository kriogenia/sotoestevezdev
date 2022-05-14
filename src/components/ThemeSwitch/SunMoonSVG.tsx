import { useTranslation } from "react-i18next";
import styled from "styled-components";
import theme from "styled-theming";
import { hoverPrimaryColor, primaryColor } from "../../theme/colors";
import { SVG } from "../../theme/styles";

const SunMoon = styled(SVG)`
  inline-size: 100%;
  block-size: 100%;
  stroke-linecap: round;
`;

const sunTransform = theme("theme", {
  light: "scale(1)",
  dark: "scale(1.75)",
});

const beamsOpacity = theme("theme", {
  light: "100",
  dark: "0",
});

const moonTransform = theme("theme", {
  light: "translateX(0px)",
  dark: "translateX(-7px)",
});

const Sun = styled.circle`
  transform-origin: center center;
  fill: ${primaryColor};
  transform: ${sunTransform};

  :hover,
  :focus-visible {
    fill: ${hoverPrimaryColor};
  }

  transition: transform 0.5s ease;
`;

const Beams = styled.g`
  transform-origin: center center;
  stroke: ${primaryColor};
  stroke-width: 2px;
  opacity: ${beamsOpacity};

  :hover,
  :focus-visible {
    stroke: ${hoverPrimaryColor};
  }

  transition: transform 0.5s ease, opacity 0.5s ease;
`;

const Moon = styled.mask`
  transform-origin: center center;
  fill: ${primaryColor};

  & > circle {
    transform: ${moonTransform};
    transition: transform 0.25s ease;

    @supports (cx: 1) {
      transform: translateX(0);
      cx: 17;
      transition: cx 0.25s ease;
    }
  }

  :hover,
  :focus-visible {
    fill: ${hoverPrimaryColor};
  }
`;

const SunMoonSVG = () => (
  <SunMoon viewBox="0 0 24 24">
    <title>Switch</title>
    <Sun cx="12" cy="12" r="6" mask="url(#moon-mask)" fill="currentColor" />
    <Beams stroke="currentColor">
      <line x1="12" y1="1" x2="12" y2="3" />
      <line x1="12" y1="21" x2="12" y2="23" />
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" />
      <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
      <line x1="1" y1="12" x2="3" y2="12" />
      <line x1="21" y1="12" x2="23" y2="12" />
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" />
      <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
    </Beams>
    <Moon id="moon-mask">
      <rect x="0" y="0" width="100%" height="100%" fill="white" />
      <circle cx="24" cy="10" r="6" fill="black" />
    </Moon>
  </SunMoon>
);

export default SunMoonSVG;
