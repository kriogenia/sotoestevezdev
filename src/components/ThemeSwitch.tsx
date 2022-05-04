import { useDispatch } from "react-redux";
import { toggleTheme } from "../reducers";

/// TODO change aspect
const ThemeSwitch = () => {
  const dispatch = useDispatch();
  
  return (
    <p>
      <input type="checkbox" onClick={() => dispatch(toggleTheme())}></input> Dark Theme
    </p>
  );
};

export default ThemeSwitch;
