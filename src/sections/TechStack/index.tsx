import React, { useState } from "react";
import { useTranslation } from "react-i18next";
import SectionContainer from "../../components/SectionContainer";
import Slide from "../../components/Slide";
import { ITech } from "./IStack";
import Stack from "./Stack";
import data from "./stack.json";

const ns = "techstack";

const stacks = Object.entries(data).map(([key, value]) => {
  const techs = value as unknown as ITech[];
  return { key, techs };
});

const TechStack = () => {
  const [current, setCurrent] = useState(0);

  const { t } = useTranslation(ns);

  return (
    <SectionContainer title={t("title")} ns={ns}>
      <Slide i={current} set={setCurrent} length={stacks.length}>
        <Stack stack={stacks[current]} />
      </Slide>
    </SectionContainer>
  );
};

export default TechStack;
