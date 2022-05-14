import { createSlice, PayloadAction } from '@reduxjs/toolkit'

const LANG_TOGGLE = "lang_toggle";

export interface State {
	lang: string
}

const langSlice = createSlice({
  name: LANG_TOGGLE,
  initialState: {
	  lang: "en"
  },
  reducers: {
    change(state: State, action: PayloadAction<string>) {
      state.lang = action.payload;
    }
  }
})

export const { change } = langSlice.actions
export default langSlice.reducer