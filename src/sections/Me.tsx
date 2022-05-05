import React from "react";
import { Trans, useTranslation } from "react-i18next";
import styled from "styled-components";
import SectionContainer from "../components/SectionContainer";
import { primaryColor } from "../theme/colors";
import { Title } from "../theme/styles";
import { ReactComponent as Signing } from "./img/signing.svg";

const ns = "intro";

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
`;
const Photo = styled.img`
  height: 25vh;
`;

const Me = () => {
  const { t } = useTranslation(ns);
  return (
    <SectionContainer ns="intro">
      <Photo src={process.env.PUBLIC_URL + "/img/me.png"} alt={t("me_alt")} />
      <Name>
        Ricardo <Surname>Soto Estévez</Surname>
      </Name>
      <p>
        <Trans i18nKey="subtitle" ns={ns}>
          Good is not the word I would to describe myself as developer, it is '
          <Emphasis>Caralludo</Emphasis>'
        </Trans>
      </p>
		<Signing/>
    </SectionContainer>
  );
};

export default Me;
