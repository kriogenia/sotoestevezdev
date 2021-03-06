import React, { FC, ReactNode, useEffect, useState } from "react";
import styled from "styled-components";
import i18n from "../i18n";
import { primaryColor } from "../theme/colors";
import { hideMobile, SVG, Title } from "../theme/styles";

const Container = styled.div`
  min-height: 110vh;
  display: flex;
  flex-direction: row;
  justify-content: start;
  align-items: center;
`;

const Column = styled.div`
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
`;

const Side = styled(Column)`
  margin: 10px;
  flex-direction: row;
  width: 10vw;

  ${hideMobile}
`;

const Content = styled(Column)`
  width: 100%;
  flex-direction: column;
`;

const SectionTitle = styled(Title)`
  text-transform: uppercase;
  font-weight: bold;
`;

const CircleSVG = styled(SVG)`
  width: 40px;
  height: 40px;
  margin: 5px;
  fill: ${primaryColor};
`;

interface Props {
  title: string;
  ns: string;
  children: ReactNode;
}

const SectionContainer: FC<Props> = ({ title, ns, children }) => {
  const [tLoaded, setTLoaded] = useState(false);

  useEffect(() => {
    i18n.on("languageChanged", () => setTLoaded(false));
    if (!tLoaded) {
      i18n
        .loadNamespaces(ns)
        .then(() => {
          setTLoaded(true);
        })
        .catch(console.error);
    }
  }, [ns, tLoaded]);
  return (
    <Container>
      {tLoaded && (
        <>
          {title && (
            <Side>
              <CircleSVG viewBox="0 0 10 10">
                <circle cx="5" cy="5" r="5" />
              </CircleSVG>
              <SectionTitle>{title}</SectionTitle>
            </Side>
          )}
          <Content>{children}</Content>
		  { title && (
			  <Side/>
		  )}
        </>
      )}
    </Container>
  );
};

export default SectionContainer;
