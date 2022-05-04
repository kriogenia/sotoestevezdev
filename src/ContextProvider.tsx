import React, { FC, ReactNode } from "react";
import { Provider, useSelector } from "react-redux";
import { ThemeProvider } from "styled-components";
import store, { IState } from "./store";

interface Props {
  children: ReactNode;
}

const MyThemeProvider: FC<Props> = ({ children }) => {
  const isDark = useSelector((state: IState) => state.theme.isDark);
  return (
    <ThemeProvider theme={{ theme: isDark ? "dark" : "light" }}>
      {children}
    </ThemeProvider>
  );
};
const ContextProvider: FC<Props> = ({ children }) => {
  return (
    <Provider store={store}>
      <MyThemeProvider>{children}</MyThemeProvider>
    </Provider>
  );
};

export default ContextProvider;
