import React from "react";
import { Trans, useTranslation } from "react-i18next";
import styled from "styled-components";
import Divider from "../../components/Divider";
import SectionContainer from "../../components/SectionContainer";
import { frontColor, primaryColor } from "../../theme/colors";
import NetworkLinks from "./NetworkLinks";

const ns = "intro";

const Description = styled.div`
  width: 40%;
  text-align: left;

  @media (max-width: 30em) {
    width: 100%;
  }

  & > p {
    margin: 10px;
    & > strong {
      color: ${primaryColor};
    }
  }
`;

const LinkDivider = styled(Divider)`
  height: 10px;
  fill: ${frontColor};
`;

const Introduction = () => {
  const { t } = useTranslation(ns);

  return (
    <SectionContainer title={t("title")} ns={ns}>
      <Description>
        <Trans i18nKey="description" ns={ns}>
          Who I am
        </Trans>
      </Description>
	  {/* TODO add CV and email */}
	  <LinkDivider/>
	  <NetworkLinks t={t}/>
    </SectionContainer>
  );
};

export default Introduction;
