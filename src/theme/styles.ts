import styled from "styled-components";
import { primaryColor, hoverPrimaryColor } from "./colors";

export const FlexRow = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: center;
  align-items: center;
`;

export const Link = styled.a`
  color: ${primaryColor};
  font-weight: bolder;
  text-decoration-style: dotted;
  :hover {
    color: ${hoverPrimaryColor};
  }
`;

export const Title = styled.h1`
  margin-bottom: 0.5rem;
  font-size: 3rem;
  font-weight: 500;
  line-height: 1.2;
`;

export const Title2 = styled.h2`
  margin-bottom: 0.5rem;
  font-size: 2.75rem;
  line-height: 1.2;
`;

export const SVG = styled.svg.attrs({
  version: "1.1",
  xmlns: "http://www.w3.org/2000/svg",
  xmlnsXlink: "http://www.w3.org/1999/xlink",
  role: "img",
})``;

export const hideMobile = `
  @media only screen and (max-width: 1000px) {
	display:none;
  }`;

export const topSize = "2rem";
