import React from "react";
import { useTranslation } from "react-i18next";
import styled from "styled-components";
import SectionContainer from "../../components/SectionContainer";
import { backgroundColor, frontColor, primaryColor } from "../../theme/colors";
import { MediumIcon } from "../../theme/icons";
import MediumList from "./MediumList";

const ns = "blog";

const MoreLink = styled.a`
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 10px 10px;
  gap: 10px;

  color: ${primaryColor};
  font-weight: bolder;
  text-decoration: none;

  border: solid 2px ${primaryColor};
  border-radius: 5px;

  & > svg {
    fill: ${primaryColor};
	height: 25px;
  }

  :hover {
    color: ${backgroundColor};
    background-color: ${primaryColor};
    & > svg {
      fill: ${backgroundColor};
    }
  }
`;

const Title = styled.h3`
  color: ${frontColor};
  margin-bottom: 20px;
`;

const Blog = () => {
  const { t } = useTranslation(ns);

  return (
    <SectionContainer title={t("title")} ns={ns}>
	  <Title>{t("latest")}</Title>
      <MediumList />
	  <MoreLink href="https://medium.com/@sotoestevez">
		<MediumIcon/>
		{t("more")}
	  </MoreLink>
    </SectionContainer>
  );
};

export default Blog;
