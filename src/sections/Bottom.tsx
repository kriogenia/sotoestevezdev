import styled from "styled-components";
import { HorizontalDivider } from "../components/Divider";
import { backgroundColor, primaryColor } from "../theme/colors";

const Container = styled.div`
  width: 100%;
  margin-top: 100px;
  padding: 20px 0px;
  background-color: ${primaryColor};

  color: ${backgroundColor};
  font-weight: bold;

  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
`;

const Divider = styled(HorizontalDivider)`
  height: 15px;
  fill: ${backgroundColor};
`;

const Email = styled.a`
  color: ${backgroundColor};
  text-decoration-style: dotted;
`;

const Bottom = () => {
  return (
    <Container>
      <Divider />
      <p>
        {new Date().getFullYear()} - <strong>Ricardo Soto Estévez</strong>{" - "}
        <Email href="mailto:ricardo@sotoestevez.dev">
          ricardo@sotoestevez.dev
        </Email>
      </p>
      <Divider />
    </Container>
  );
};

export default Bottom;
