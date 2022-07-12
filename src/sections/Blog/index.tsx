import React from "react";
import { Trans, useTranslation } from "react-i18next";
import styled from "styled-components";
import SectionContainer from "../../components/SectionContainer";
import { primaryColor } from "../../theme/colors";
import MediumList from "./MediumList";

const ns = "blog";

const Description = styled.div`
  width: 50%;
  text-align: left;

  @media only screen and (max-width: 1000px) {
    width: 100%;
  }

  & > p {
    margin: 10px;
    & > strong {
      color: ${primaryColor};
    }
  }
`;

const Blog = () => {
  const { t } = useTranslation(ns);

  return (
    <SectionContainer title={t("title")} ns={ns}>
      <MediumList />
    </SectionContainer>
  );
};

export default Blog;
