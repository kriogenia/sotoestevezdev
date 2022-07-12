import React from "react";
import { Trans, useTranslation } from "react-i18next";
import styled from "styled-components";
import { FrontHorizontalDivider } from "../../components/Divider";
import SectionContainer from "../../components/SectionContainer";
import { backgroundColor, primaryColor } from "../../theme/colors";
import CvIcon from "./CvIcon";
import NetworkLinks from "./NetworkLinks";

const ns = "intro";

const Description = styled.div`
  width: 50%;
  text-align: left;

  @media only screen and (max-width: 1000px) {
    width: 100%;
  }

  & > p {
    margin: 10px;
    & > strong {
      color: ${primaryColor};
    }
  }
`;

const IntroDivider = styled(FrontHorizontalDivider)`
  height: 10px;
  margin: 20px;
`;

const CVLink = styled.a`
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 10px 5px;
  gap: 5px;

  color: ${primaryColor};
  font-weight: bolder;
  text-decoration: none;

  border: solid 2px ${primaryColor};
  border-radius: 5px;

  & > svg {
    fill: ${primaryColor};
	height: 25px;
  }

  :hover {
    color: ${backgroundColor};
    background-color: ${primaryColor};
    & > svg {
      fill: ${backgroundColor};
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
      <IntroDivider />
      <CVLink
        download
        href={process.env.PUBLIC_URL + "/docs/RicardoSotoEstevez.pdf"}
      >
        <CvIcon />
        Curriculum Vitae
      </CVLink>
      <IntroDivider />
      <NetworkLinks t={t} />
    </SectionContainer>
  );
};

export default Introduction;
