import { FC } from "react";
import { TFunction } from "react-i18next";
import styled from "styled-components";
import { primaryColor } from "../../theme/colors";
import icons from "../../theme/icons";
import { Link } from "../../theme/styles";
import { ILink } from "./IProject";

const Container = styled.div`
  display: flex;
  align-items: center;

  & > svg {
    height: 20px;
    fill: ${primaryColor};
    margin: 5px;
  }
`;

const Ref = styled(Link)``;

interface Props {
  link: ILink;
  t: TFunction;
}

const ProjectLink: FC<Props> = ({ link, t }) => {
  return (
    <Container>
      {icons[link.icon as keyof typeof icons]}
      <Ref href={link.url} download={link.download}>
        {t(`links.${link.key}`)}
      </Ref>
    </Container>
  );
};

export default ProjectLink;
