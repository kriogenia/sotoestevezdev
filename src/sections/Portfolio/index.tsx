import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import styled from "styled-components";
import SectionContainer from "../../components/SectionContainer";
import Slide from "../../components/Slide";
import { frontColor } from "../../theme/colors";
import Project from "./Project";
import data from "./projects.json";

const ns = "portfolio";

const Index = styled.p`
  font-size: 1.25rem;
  margin: 10px;
  color: ${frontColor};
`;

const projects = Object.entries(data).map(([key, project]) => {
  return { key, ...project };
});

const Portfolio = () => {
  const [current, setCurrent] = useState(0);

  const { t } = useTranslation(ns);

  return (
    <SectionContainer title={t("title")} ns={ns}>
      <Slide length={projects.length} set={setCurrent} i={current}>
        <Project project={projects[current]} ns={ns} />
      </Slide>
      <Index>
        <em>{`${current + 1}/${projects.length}`}</em>
      </Index>
    </SectionContainer>
  );
};

export default Portfolio;
