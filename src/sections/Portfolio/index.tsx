import React from "react";
import { useTranslation } from "react-i18next";
import SectionContainer from "../../components/SectionContainer";
import Project from "./Project";
import projects from "./projects.json";

const ns = "portfolio";

const Portfolio = () => {
  const { t } = useTranslation(ns);

  return (
    <SectionContainer title="Portfolio" ns={ns}>
      {Object.entries(projects).map(([key, project]) => (
        <Project key={key} project={{key, ...project}} t={t} />
      ))}
    </SectionContainer>
  );
};

export default Portfolio;
