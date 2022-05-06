import styled from "styled-components";
import { primaryColor } from "../../theme/colors";
import { SVG } from "../../theme/styles";

const Signing = styled(SVG)`
  stroke-linecap: round;
`;

const Name = styled.path`
  fill-opacity: 0;
  stroke: ${primaryColor};
  stroke-width: 3;
  stroke-dasharray: 1459;
  stroke-dashoffset: 1459;
  animation: sign-name 10s ease infinite;

  @keyframes sign-name {
    0% {
      stroke-dashoffset: -1459;
    }
    40% {
      stroke-dashoffset: 0;
    }
    85% {
      stroke-dashoffset: 0;
    }
    95% {
      stroke-dashoffset: -1459;
    }
  }
`;

const Initial = styled.path`
  fill-opacity: 0;
  stroke: ${primaryColor};
  stroke-width: 4;
  stroke-dasharray: 1198;
  stroke-dashoffset: 1198;
  animation: sign-initial 10s ease infinite;

  @keyframes sign-initial {
    0% {
      stroke-dashoffset: -1198;
    }
    40% {
      stroke-dashoffset: -1198;
    }
    50% {
      stroke-dashoffset: 0;
    }
    85% {
      stroke-dashoffset: 0;
    }
    95% {
      stroke-dashoffset: -1198;
    }
  }
`;

const SigningSVG = () => {
  return (
    <Signing viewBox="0 0 640 640" width="250" height="250">
      <Name
        d="M391.29 481.94C395.28 503.66 414.41 493.58 432.74 454.02C451.08 
		414.47 447.09 368.71 391.29 471.08C346.65 552.99 446.05 100.37 450.71 
		75.59C455.37 50.81 422.22 192.34 412.81 234.53C400.82 288.32 382.04 359.89 
		375.93 387.67C368.61 420.95 359.14 450.68 354.52 461.83C347.88 477.87 342.28 
		484.97 335.26 493.17C328.25 501.37 328.68 396.73 315.53 416.83C313.43 420.05 
		301.52 424.64 299.73 426.75C290.35 437.8 293.89 436.33 275.14 478.03C252.82 
		527.67 269.37 329.97 260.6 416.83C254.75 474.75 235.34 493.74 202.36 473.81"
      />
      <Initial
        d="M566.89 378.76C465.25 515.92 318.77 540.53 127.45 452.61C121.71 442.71 
		112.17 443.38 98.84 454.61C78.85 471.46 25.61 547.64 174.93 409.35C370.12 228.58 
		328.62 143.82 50.43 155.08"
      />
    </Signing>
  );
};

export default SigningSVG;
