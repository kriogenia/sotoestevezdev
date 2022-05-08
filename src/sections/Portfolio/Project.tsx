import { TFunction } from "react-i18next";
import React, { FC } from "react";
import { IProject } from "./IProject";
import styled from "styled-components";

const Container = styled.div`
	width: 60%;

	@media (hover: none) {
	  width: 100%;
	}
`;

interface Props {
  project: IProject;
  t: TFunction;
}

const Project: FC<Props> = ({ project, t }) => {
  const key = project.key;
  return (
    <Container>
      <h2>{t(`${key}.name`)}</h2>
      <p>{t(`${key}.description`)}</p>
      {project.links.map((link) => (
        <p>
          {`${link.icon} - ${link.key} - ${link.url}`}
        </p>
      ))}
      {project.tags.map((tag) => (
        <p>{tag}</p>
      ))}
    </Container>
  );
};

export default Project;
