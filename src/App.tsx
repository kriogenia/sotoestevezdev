import React from "react";
import styled from "styled-components";
import theme from "styled-theming";
import Scroll from "./Scroll";
import ContextProvider from "./ContextProvider";

export const backgroundColor = theme("theme", {
  light: "#fff",
  dark: "#2d2d2d",
});

export const textColor = theme("theme", {
  light: "#000",
  dark: "#fff",
});

const Container = styled.div`
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