import { FC } from "react";
import styled from "styled-components";
import { frontColor } from "../theme/colors";
import { SVG } from "../theme/styles";

const DividerSVG = styled(SVG)``;

interface Props {
  className?: string;
}

export const HorizontalDivider: FC<Props> = ({ className }) => {
  return (
    <DividerSVG viewBox="0 0 64 10" className={className}>
      <circle r="2.5" cx="25%" cy="50%" />
      <circle r="5" cx="50%" cy="50%" />
      <circle r="2.5" cx="75%" cy="50%" />
    </DividerSVG>
  );
};

export const LinearDivider: FC<Props> = ({ className }) => {
  return (
    <DividerSVG viewBox="0 0 10 10" className={className}>
      <circle r="5" cx="50%" cy="50%" />
    </DividerSVG>
  );
};

export const FrontHorizontalDivider = styled(HorizontalDivider)`
  fill: ${frontColor};
`;
