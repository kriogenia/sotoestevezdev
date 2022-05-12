import React, { Dispatch, FC, ReactNode, SetStateAction } from "react";
import styled from "styled-components";
import { frontColor } from "../theme/colors";
import { BackIcon, ForwardIcon } from "../theme/icons";
import { FlexRow } from "../theme/styles";

const Container = styled(FlexRow)`
  width: 100%;
`;

const Button = styled.button`
  --size: 8rem;

  background: none;
  border: none;
  padding: 0;

  inline-size: var(--size);
  aspect-ratio: 1;

  cursor: pointer;
  touch-action: manipulation;
  -webkit-tap-highlight-color: transparent;
  outline-offset: 5px;

  transition: all 0.3s linear;
  :hover {
    transform: scale(1.5);
  }

  @media (hover: none) {
    --size: 36px;
  }
`;

const Back = styled(BackIcon)`
  fill: ${frontColor};
  height: 60px;

  @media (hover: none) {
    height: 36px;
  }
`;

const Next = styled(ForwardIcon)`
  fill: ${frontColor};
  height: 60px;

  @media (hover: none) {
    height: 36px;
  }
`;

interface Props {
	i: number;
	set: Dispatch<SetStateAction<number>>;
  length: number;
  children: ReactNode;
}

const Slide: FC<Props> = ({ set, i, length, children }) => {

  const next = () => {
    if (i + 1 >= length) {
      set(0);
    } else {
      set(i + 1);
    }
  };

  const prev = () => {
    if (i === 0) {
      set(length - 1);
    } else {
      set(i - 1);
    }
  };

  return (
    <Container>
      <Button onClick={prev}>
        <Back />
      </Button>
      {children}
      <Button onClick={next}>
        <Next />
      </Button>
    </Container>
  );
};

export default Slide;
