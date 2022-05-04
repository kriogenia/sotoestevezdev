import React from "react";
import styled from "styled-components";
import Scroll from "./components/Scroll";
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
    <ContextProvider>
      <Container>
        <Scroll />
      </Container>
    </ContextProvider>
  );
};

export default App;