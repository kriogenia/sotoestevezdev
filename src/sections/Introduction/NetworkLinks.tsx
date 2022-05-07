import { TFunction } from "i18next";
import React, { FC } from "react";
import styled from "styled-components";
import { FlexRow } from "../../theme/styles";
import links from "./links.json";

const Icon = styled.img`
	--size: 50px;

	height: var(--size);
	width: var(--size);
	margin: 5px;
`;

interface Props {
  t: TFunction;
}

const NetworkLinks: FC<Props> = ({ t }) => {
  return (
    <FlexRow>
      {Object.entries(links).map(([k, v]) => (
        <a href={v.url} key={k}>
          <Icon
            src={`${process.env.PUBLIC_URL}${v.img}`}
            alt={t(`alt.${k}`)}
          />
        </a>
      ))}
    </FlexRow>
  );
};

export default NetworkLinks;
