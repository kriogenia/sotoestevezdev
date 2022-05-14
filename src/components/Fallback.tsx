import RotateLoader from "react-spinners/RotateLoader";
import styled from "styled-components";

const Container = styled.div`
  width: 100%;
  height: 100%;

  display: flex;
  justify-content: center;
  align-items: center;

  & > span {
	  margin: calc(50vh - 50px)
  }
`;

const Fallback = () => {
  return (
    <Container>
      <RotateLoader color="#df691a" size={50} margin={50} speedMultiplier={0.65}/>
    </Container>
  );
};

export default Fallback;