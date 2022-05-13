import { FC } from "react";
import styled from "styled-components";
import { ITech } from "./IStack";
import filters from "./levels";

const Icon = styled.img`
  height: 75px;
  width: 75px;
  margin: 10px 0px;

  filter: ${(props: { level: number }) => filters[props.level]};
`;

const Tech = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;

  margin: 25px;

  font-weight: bold;
`;

interface Props {
  tech: ITech;
}

const TechLogo: FC<Props> = ({ tech: { key, name, level } }) => {
  return (
    <Tech key={key}>
      <Icon
        src={`${process.env.PUBLIC_URL}/img/logos/${key}.svg`}
        alt={name}
        level={level}
      />
      <span>{name}</span>
    </Tech>
  );
};

export default TechLogo;
