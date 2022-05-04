import React from "react";
import styled from "styled-components";
import SectionContainer from "../components/SectionContainer";
import { primaryColor } from "../theme/colors";
import { Title } from "../theme/styles";

const Name = styled(Title)`
	font-size: 3.5rem;
`;
const Surname = styled.span`
	color: ${primaryColor};
	text-transform: uppercase;
	font-weight: bolder;
`

const Intro = () => {
  return (
    <SectionContainer>
      <Name>
        Ricardo <Surname>Soto Estévez</Surname>
      </Name>
    </SectionContainer>
  );
};

export default Intro;
