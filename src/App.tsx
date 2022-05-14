import React, { Suspense } from "react";
import styled, { createGlobalStyle } from "styled-components";
import Fallback from "./components/Fallback";
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

	color: ${textColor};
	
	background-color: ${backgroundColor};

	background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='100%25' height='100%25' viewBox='0 0 16 8'%3E %3Ccircle fill='%23000' r='10' cx='-50%25' cy='50%25' opacity='0.1' /%3E %3Ccircle fill='%23000' r='2' cx='16' cy='-1' opacity='0.2' /%3E %3Cg%3E %3Ccircle fill='%23fff' r='4%25' cx='5%25' cy='50%25' opacity='0.15'/%3E %3Ccircle fill='%23fff' r='1%25' cx='5%25' cy='25%25' opacity='0.10'/%3E %3Ccircle fill='%23fff' r='1%25' cx='5%25' cy='75%25' opacity='0.10'/%3E %3C/g%3E  %3C/svg%3E");
	background-repeat: no-repeat;
	background-size: cover;
	background-position: bottom center, 50%, 50%;
	background-attachment: fixed;
`;

const App = () => {
  return (
    <ContextProvider>
      <GlobalStyle />
      <Suspense fallback={<Fallback />}>
        <Container>
          <Top />
          <Scroll />
        </Container>
      </Suspense>
    </ContextProvider>
  );
};

export default App;
