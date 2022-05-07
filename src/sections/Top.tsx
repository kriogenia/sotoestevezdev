import styled from "styled-components";
import { LinearDivider } from "../components/Divider";
import LanguageSelector from "../components/LanguageSelector";
import ThemeSwitch from "../components/ThemeSwitch";
import { frontColor } from "../theme/colors";

const Container = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: end;
  padding: 10px;
  gap: 15px;
`;

const Dot = styled(LinearDivider)`
	fill: ${frontColor};
	height: 5px;
`;

const Top = () => {
  return (
    <Container>
      <LanguageSelector />
      <Dot />
      <ThemeSwitch />
    </Container>
  );
};

export default Top;
