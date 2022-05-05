import React, { Suspense } from "react";
import styled from "styled-components";
import LanguageSelector from "./components/LanguageSelector";
import Scroll from "./components/Scroll";
import ThemeSwitch from "./components/ThemeSwitch";
import ContextProvider from "./ContextProvider";
import { backgroundColor, textColor } from "./theme/colors";

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
    <Suspense fallback="loading">
      <ContextProvider>
        <Container>
          <LanguageSelector />
          <ThemeSwitch />
          <Scroll />
        </Container>
      </ContextProvider>
    </Suspense>
  );
};

export default App;
