import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import styled from "styled-components";
import SectionContainer from "../../components/SectionContainer";
import { frontColor } from "../../theme/colors";
import { BackIcon, ForwardIcon } from "../../theme/icons";
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

const Container = styled(FlexRow)`
  width: 100%;
`;

const Button = styled.button`
  --size: 8rem;

  background: none;
  border: none;
  padding: 0;

  inline-size: var(--size);
  aspect-ratio: 1;

  cursor: pointer;
  touch-action: manipulation;
  -webkit-tap-highlight-color: transparent;
  outline-offset: 5px;

  transition: all 0.3s linear;
  :hover {
    transform: scale(1.5);
  }

  @media (hover: none) {
    --size: 36px;
  }
`;

const Back = styled(BackIcon)`
  fill: ${frontColor};
  height: 60px;

  @media (hover: none) {
    height: 36px;
  }
`;

const Next = styled(ForwardIcon)`
  fill: ${frontColor};
  height: 60px;

  @media (hover: none) {
    height: 36px;
  }
`;

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
      <Container>
        <Button onClick={prev}>
          <Back />
        </Button>
        <Project project={projects[current]} t={t} />
        <Button onClick={next}>
          <Next />
        </Button>
      </Container>
    </SectionContainer>
  );
};

export default Portfolio;
