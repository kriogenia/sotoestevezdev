import { FC } from "react";
import styled from "styled-components";
import { primaryFilter } from "../../theme/filters";
import { Title2 } from "../../theme/styles";
import { IStack } from "./IStack";

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

const Icon = styled.img`
  height: 75px;
  width: 75px;
  margin: 10px 0px;

  filter: ${primaryFilter};
`;

const Tech = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;

  margin: 25px;

  font-weight: bold;
`;

interface Props {
  stack: IStack;
}

const Stack: FC<Props> = ({ stack }) => {
  return (
    <Container>
      <Title>{stack.key}</Title>
      <TechsContainer>
        {stack.techs.map(({ key, name }) => (
          <Tech key={key}>
            <Icon
              src={`${process.env.PUBLIC_URL}/img/logos/${key}.svg`}
              alt={name}
            />
            <span>{name}</span>
          </Tech>
        ))}
      </TechsContainer>
    </Container>
  );
};

export default Stack;
