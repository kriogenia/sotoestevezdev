import { Trans, useTranslation } from "react-i18next";
import React, { FC } from "react";
import { IProject } from "./IProject";
import styled from "styled-components";
import Link from "./ProjectLink";
import Tag from "./ProjectTag";
import { frontColor, primaryColor } from "../../theme/colors";
import { Title2 } from "../../theme/styles";

const Container = styled.div`
  width: 60%;
  text-align: left;

  @media (hover: none) {
    width: 100%;
  }
`;

const Subtitle = styled.h3`
  color: ${frontColor};
  margin-bottom: 20px;
`;

const Description = styled.p`
  margin: 40px 0px;

  & > strong {
	  color: ${primaryColor};
  }

  @media (hover: none) {
    margin: 10px;
  }
`;

const Links = styled.div`
  margin: 25px;
`;

interface Props {
  project: IProject;
  ns: string;
}

const Project: FC<Props> = ({ project, ns }) => {
  const { t } = useTranslation(ns);

  const key = project.key;
  return (
    <Container>
      <Title2>{t(`${key}.name`)}</Title2>
      <Subtitle>{t(`${key}.subtitle`)}</Subtitle>
      {project.tags.map((tag) => (
        <Tag key={tag} tag={tag} />
      ))}
      <Description>
        <Trans i18nKey={`${key}.description`} ns={ns}></Trans>
      </Description>
      <Links>
        {project.links.map((link) => (
          <Link key={link.key} link={link} t={t} />
        ))}
      </Links>
    </Container>
  );
};

export default Project;
