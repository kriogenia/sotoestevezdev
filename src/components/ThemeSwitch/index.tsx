import { useDispatch } from "react-redux";
import styled from "styled-components";
import { toggleTheme } from "../../reducers";
import SunMoonSVG from "./SunMoonSVG";

const Button = styled.button`
  --size: 2rem;

  background: none;
  border: none;
  padding: 0;

  inline-size: var(--size);
  block-size: var(--size);
  aspect-ratio: 1;
  border-radius: 50%;

  cursor: pointer;
  touch-action: manipulation;
  -webkit-tap-highlight-color: transparent;
  outline-offset: 5px;

  @media (hover: none) {
    --size: 48px;
  }
`;

/// TODO change aspect
const ThemeSwitch = () => {
  const dispatch = useDispatch();

  return (
    <Button onClick={() => dispatch(toggleTheme())}>
      <SunMoonSVG />
    </Button>
  );
};

export default ThemeSwitch;
