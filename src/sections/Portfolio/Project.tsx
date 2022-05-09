import { TFunction } from "react-i18next";
import React, { FC } from "react";
import { IProject } from "./IProject";
import styled from "styled-components";
import Link from "./ProjectLink";
import Tag from "./ProjectTag";
import { frontColor } from "../../theme/colors";

const Container = styled.div`
  width: 60%;
  text-align: left;

  @media (hover: none) {
    width: 100%;
  }
`;

const Title = styled.h2`
  font-size: xxx-large;
`;

const Subtitle = styled.h3`
  color: ${frontColor};
  margin-bottom: 20px;
`;

const Description = styled.p`
  margin: 40px 0px;

  @media (hover: none)  {
	  margin: 10px;
  }
`;

const Links = styled.div`
  margin: 25px;
`;

interface Props {
  project: IProject;
  t: TFunction;
}

const Project: FC<Props> = ({ project, t }) => {
  const key = project.key;
  return (
    <Container>
      <Title>{t(`${key}.name`)}</Title>
      <Subtitle>{t(`${key}.subtitle`)}</Subtitle>
      {project.tags.map((tag) => (
        <Tag key={tag} tag={tag} />
      ))}
      <Description>{t(`${key}.description`)}</Description>
      <Links>
        {project.links.map((link) => (
          <Link key={link.key} link={link} t={t} />
        ))}
      </Links>
    </Container>
  );
};

export default Project;
