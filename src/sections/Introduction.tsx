import React from "react";
import { Trans, useTranslation } from "react-i18next";
import styled from "styled-components";
import SectionContainer from "../components/SectionContainer";
import { primaryColor } from "../theme/colors";

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

const Introduction = () => {
  const { t } = useTranslation(ns);

  return (
    <SectionContainer title={t("title")} ns={ns}>
      <Description>
        <Trans i18nKey="description" ns={ns}>
          Who I am
        </Trans>
      </Description>
    </SectionContainer>
  );
};

export default Introduction;
