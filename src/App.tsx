import React /*, { Suspense }*/ from "react";
import styled, { createGlobalStyle } from "styled-components";
import Scroll from "./components/Scroll";
import ContextProvider from "./ContextProvider";
import { Top } from "./sections";
import { backgroundColor, textColor } from "./theme/colors";

const GlobalStyle = createGlobalStyle`
  body {
	background-color: ${backgroundColor};
  }
`;

const Container = styled.div`
  height: 100%;
  margin: 0;
  border: 0;
  font-family: "Fira Sans", -apple-system, BlinkMacSystemFont, "Segoe UI",
    "Roboto", "Oxygen", "Ubuntu", "Cantarell", "Droid Sans", "Helvetica Neue",
    sans-serif;
  background-color: ${backgroundColor};
  color: ${textColor};
`;

const App = () => {
  return (
    //<Suspense fallback="loading">
    <ContextProvider>
      <GlobalStyle />
      <Container>
        <Top />
        <Scroll />
      </Container>
    </ContextProvider>
    //</Suspense>
  );
};

export default App;
