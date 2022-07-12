import { FC } from "react";
import { TFunction } from "react-i18next";
import styled from "styled-components";
import { Title2 } from "../../theme/styles";
import { IStack } from "./IStack";
import Tech from "./TechLogo";

const Container = styled.div`
  width: 60%;

  @media (hover: none) {
    width: 100%;
  }
`;

const Title = styled(Title2)`
  margin-bottom: 50px;
  text-transform: uppercase;
`;

const TechsContainer = styled.div`
  max-width: 60%;
  margin: 0 20%;

  @media (hover: none) {
    max-width: 100%;
	margin: 0;
  }

  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  justify-content: center;
`;

interface Props {
  stack: IStack;
  t: TFunction;
}

const Stack: FC<Props> = ({ stack, t }) => {
  return (
    <Container>
      <Title>{t(`section.${stack.key}`)}</Title>
      <TechsContainer>
        {stack.techs.map((tech) => (
          <Tech key={tech.key} tech={tech}/>
        ))}
      </TechsContainer>
    </Container>
  );
};

export default Stack;
