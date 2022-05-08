import { FC } from "react";
import styled from "styled-components";
import { SVG } from "./styles";

interface Props {
  className?: string;
}

const Base = styled(SVG)`
  stroke-linecap: round;
`;

export const BackIcon: FC<Props> = ({ className }) => {
  return (
    <Base viewBox="0 0 18 24" className={className}>
      <path d="M11.67 3.87 9.9 2.1 0 12l9.9 9.9 1.77-1.77L3.54 12z"></path>
	  <circle cx="14" cy="12" r="2"></circle>
    </Base>
  );
};

export const ForwardIcon: FC<Props> = ({ className }) => {
  return (
    <Base viewBox="0 0 18 24" className={className}>
      {/*<path d="M6.23 20.23 8 22l10-10L8 2 6.23 3.77 14.46 12z"></path>*/}
      <path d="M6.23 20.23 8 22l10-10L8 2 6.23 3.77 14.46 12z"></path>
	  <circle cx="4" cy="12" r="2"></circle>
    </Base>
  );
};
