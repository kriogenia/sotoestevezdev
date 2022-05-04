import { createSlice } from '@reduxjs/toolkit'

const THEME_TOGGLE = "theme_toggle";

export interface State {
	isDark: boolean
}

const themeSlice = createSlice({
  name: THEME_TOGGLE,
  initialState: {
	  isDark: true
  },
  reducers: {
    toggle(state) {
      state.isDark = !state.isDark;
    }
  }
})

export const { toggle } = themeSlice.actions
export default themeSlice.reducer