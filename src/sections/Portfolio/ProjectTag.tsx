import { FC } from "react";
import styled from "styled-components";
import { tagColor, backgroundColor } from "../../theme/colors";
import { backgroundFilter } from "../../theme/filters";

const Tag = styled.span`
  display: inline-flex;
  align-items: center;
  text-align: center;
  gap: 5px;

  background-color: ${tagColor};
  border-radius: 5px;

  margin: 5px 2px;
  padding: 5px;

  color: ${backgroundColor};
  font-weight: bold;
`;

const Icon = styled.img`
  --size: 25px;
  height: var(--size);
  width: var(--size);

  filter: ${backgroundFilter};
`;

interface Props {
  tag: string;
}

const ProjectTag: FC<Props> = ({ tag }) => {
  return (
    <Tag>
      <Icon
        src={`${process.env.PUBLIC_URL}/img/logos/${tag}.svg`}
        alt={`${tag} logo`}
      />
      {tag}
    </Tag>
  );
};

export default ProjectTag;
