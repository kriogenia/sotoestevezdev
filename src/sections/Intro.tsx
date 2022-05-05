import React from "react";
import { Trans } from "react-i18next";
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
`;
const Emphasis = styled.em`
	font-weight: bold;
	font-family: "Roboto";
`

const Intro = () => {
  //c onst { t } = useTranslation();
  return (
    <SectionContainer>
      <Name>
        Ricardo <Surname>Soto Estévez</Surname>
      </Name>
	  <p>
		  <Trans i18nKey="intro:subtitle">
			  Good is not the word I would to describe myself as developer, it is '<Emphasis>Caralludo</Emphasis>'
		  </Trans>
	  </p>
    </SectionContainer>
  );
};

export default Intro;
