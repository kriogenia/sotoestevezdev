import { FC } from "react";

interface Props {
  tag: string;
}

const ProjectTag: FC<Props> = ({ tag }) => {
  return <span>{tag}</span>;
};

export default ProjectTag;
