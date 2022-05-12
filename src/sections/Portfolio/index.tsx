import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import SectionContainer from "../../components/SectionContainer";
import Slide from "../../components/Slide";
import Project from "./Project";
import data from "./projects.json";

const ns = "portfolio";

const projects = Object.entries(data).map(([key, project]) => {
  return { key, ...project };
});

const Portfolio = () => {
	const [current, setCurrent] = useState(0);

	const { t } = useTranslation();

  return (
    <SectionContainer title={t("title")} ns={ns}>
		<Slide length={projects.length} set={setCurrent} i={current}>
        <Project project={projects[current]} ns={ns}/>
		</Slide>
    </SectionContainer>
  );
};

export default Portfolio;
