import React from "react";
import { Trans, useTranslation } from "react-i18next";
import styled from "styled-components";
import SectionContainer from "../../components/SectionContainer";
import { primaryColor } from "../../theme/colors";
import { Title } from "../../theme/styles";
import SigningSVG from "./SigningSVG";
import Portrait from "./me.png";

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
  --size: 250px;

  height: var(--size);
  width: var(--size);
`;

const Me = () => {
  const { t } = useTranslation(ns);
  return (
    <SectionContainer title="" ns="intro">
      <Photo src={Portrait} alt={t("alt.me")} />
      <Name>
        Ricardo <Surname>Soto Estévez</Surname>
      </Name>
      <p>
        <Trans i18nKey="subtitle" ns={ns}>
          Good is not the word I would use to describe myself as developer, it
          is '<Emphasis>Caralludo</Emphasis>'
        </Trans>
      </p>
      <SigningSVG />
    </SectionContainer>
  );
};

export default Me;
