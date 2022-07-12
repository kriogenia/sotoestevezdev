import React from "react";
import { useTranslation } from "react-i18next";
import SectionContainer from "../../components/SectionContainer";
import MediumList from "./MediumList";

const ns = "blog";

const Blog = () => {
  const { t } = useTranslation(ns);

  return (
    <SectionContainer title={t("title")} ns={ns}>
      <MediumList />
    </SectionContainer>
  );
};

export default Blog;
