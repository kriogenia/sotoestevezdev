import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import styled from "styled-components";
import { LinearDivider } from "../../components/Divider";
import SectionContainer from "../../components/SectionContainer";
import Slide from "../../components/Slide";
import { ITech } from "./IStack";
import Stack from "./Stack";
import data from "./stack.json";
import filters from "./levels";

const ns = "techstack";

const stacks = Object.entries(data).map(([key, value]) => {
  const techs = value as unknown as ITech[];
  return { key, techs };
});

const Legend = styled.div`
	margin-top: 10px;
`;

const Level = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 5px;
`;

const LevelDot = styled(LinearDivider)`
  height: 10px;
  margin: 2px;

  filter: ${(props: { level: number }) => filters[props.level]};
`;

const TechStack = () => {
  const [current, setCurrent] = useState(0);

  const { t } = useTranslation(ns);

  return (
    <SectionContainer title={t("title")} ns={ns}>
      <Slide i={current} set={setCurrent} length={stacks.length}>
        <Stack stack={stacks[current]} t={t} />
      </Slide>
      <Legend>
        <Level>
          <p>{t("experience.high")}</p>
          {[3, 2, 1, 0].map((level) => (
            <LevelDot key={level} level={level} />
          ))}
          <p>{t("experience.low")}</p>
        </Level>
      </Legend>
    </SectionContainer>
  );
};

export default TechStack;
