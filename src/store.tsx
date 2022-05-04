import { configureStore } from "@reduxjs/toolkit";
import { IThemeState, ThemeReducer } from "./reducers";

export interface IState {
	theme: IThemeState
}

export default configureStore({
  reducer: {
    theme: ThemeReducer,
  },
});
