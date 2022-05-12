import React from "react";
import { useTranslation } from "react-i18next";
import SectionContainer from "../../components/SectionContainer";
import Stack from "./Stack";

const ns = "techstack";

const TechStack = () => {
  const { t } = useTranslation(ns);
  return (
    <SectionContainer title={t("title")} ns={ns}>
      <Stack></Stack>
    </SectionContainer>
  );
};

export default TechStack;
