import React, { FC, ReactNode, useEffect, useState } from "react";
import styled from "styled-components";
import i18n from "../i18n";

const Container = styled.div`
  height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
`;

interface Props {
  ns: string;
  children: ReactNode;
}

const SectionContainer: FC<Props> = ({ ns, children }) => {
  const [tLoaded, setTLoaded] = useState(false);

  useEffect(() => {
    i18n.on("languageChanged", () => setTLoaded(false));
    if (!tLoaded) {
      i18n.loadNamespaces(ns).then(() => {
        setTLoaded(true);
      });
    }
  }, [ns, tLoaded]);
  return <Container>{tLoaded && children}</Container>;
};

export default SectionContainer;
