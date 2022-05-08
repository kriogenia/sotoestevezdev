import { FC } from "react";
import { TFunction } from "react-i18next";
import { ILink } from "./IProject";

interface Props {
  link: ILink;
  t: TFunction;
}

const ProjectLink: FC<Props> = ({ link, t }) => {
  return <div><a href={link.url}>{t(link.key)}</a></div>;
};

export default ProjectLink;