import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import SectionContainer from "../../components/SectionContainer";
import { FlexRow } from "../../theme/styles";
import Project from "./Project";
import data from "./projects.json";

const ns = "portfolio";

const projects = Object.entries(data).map(([key, project]) => {
  return { key, ...project };
});

const index = (i: number) => {
  if (i >= projects.length) {
    return 0;
  } else if (i < 0) {
    return projects.length - 1;
  }
  return i;
};

const Portfolio = () => {
  const [current, setCurrent] = useState(0);
  const { t } = useTranslation(ns);

  const next = () => {
    setCurrent(index(current + 1));
  };

  const prev = () => {
    setCurrent(index(current - 1));
  };

  return (
    <SectionContainer title={t("title")} ns={ns}>
      <FlexRow>
        <button onClick={prev}>{t(`${projects[index(current - 1)].key}.name`)}</button>
        <Project project={projects[current]} t={t} />
        <button onClick={next}>{t(`${projects[index(current + 1)].key}.name`)}</button>
      </FlexRow>
    </SectionContainer>
  );
};

export default Portfolio;
