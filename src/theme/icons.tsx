import { FC } from "react";
import styled from "styled-components";
import { SVG } from "./styles";

interface Props {
  className?: string;
}

const Base = styled(SVG)`
  stroke-linecap: round;
`;

export const BackIcon: FC<Props> = ({ className }) => {
  return (
    <Base viewBox="0 0 18 24" className={className}>
      <path d="M11.67 3.87 9.9 2.1 0 12l9.9 9.9 1.77-1.77L3.54 12z"></path>
      <circle cx="14" cy="12" r="2"></circle>
    </Base>
  );
};

export const ForwardIcon: FC<Props> = ({ className }) => {
  return (
    <Base viewBox="0 0 18 24" className={className}>
      <path d="M6.23 20.23 8 22l10-10L8 2 6.23 3.77 14.46 12z"></path>
      <circle cx="4" cy="12" r="2"></circle>
    </Base>
  );
};

export const GitHubIcon: FC<Props> = ({ className }) => {
  return (
    <Base viewBox="0 0 24 24" className={className}>
      <title>GitHub</title>
      <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" />
    </Base>
  );
};

export const PdfIcon: FC<Props> = ({ className }) => {
  return (
    <Base viewBox="0 0 24 24" className={className}>
      <title>PDF</title>
      <path d="M20 2H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm-8.5 7.5c0 .83-.67 1.5-1.5 1.5H9v2H7.5V7H10c.83 0 1.5.67 1.5 1.5v1zm5 2c0 .83-.67 1.5-1.5 1.5h-2.5V7H15c.83 0 1.5.67 1.5 1.5v3zm4-3H19v1h1.5V11H19v2h-1.5V7h3v1.5zM9 9.5h1v-1H9v1zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm10 5.5h1v-3h-1v3z"></path>
    </Base>
  );
};

const icons = {
  github: <GitHubIcon />,
  pdf: <PdfIcon />
};

export default icons;
