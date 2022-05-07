import { FC } from "react";
import styled from "styled-components";
import { SVG } from "../theme/styles";

const DividerSVG = styled(SVG)`
  margin: 20px;
`;

const Circle = styled.circle``;

interface Props {
  className?: string;
}

const Divider: FC<Props> = ({ className }) => {
  return (
    <DividerSVG viewBox="0 0 64 10" className={className}>
      <Circle r="2.5" cx="25%" cy="50%" />
      <Circle r="5" cx="50%" cy="50%" />
      <Circle r="2.5" cx="75%" cy="50%" />
    </DividerSVG>
  );
};

export default Divider;
