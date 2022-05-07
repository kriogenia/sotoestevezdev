import { TFunction } from "i18next";
import React, { FC, useState } from "react";
import styled from "styled-components";
import { primaryFilter } from "../../theme/filters";
import { FlexRow, Link } from "../../theme/styles";
import links from "./links.json";

const email = "ricardo@sotoestevez.dev";

const Icon = styled.img`
  --size: 30px;
  height: var(--size);
  width: var(--size);

  margin: 10px;
  filter: ${primaryFilter};

  transition: all 0.3s linear;
  :hover {
    transform: scale(1.5);
  }
`;

const EmailLink = styled(Link)`
  margin: 10px;
`;

const User = styled.p`
  min-height: 1.2em;
`;

interface Props {
  t: TFunction;
}

const NetworkLinks: FC<Props> = ({ t }) => {
	const [text, setText] = useState(" ");

	const clear = () => {
		setText(" ");
	} 

  return (
    <>
      <EmailLink href={`mailto:${email}`}>{email}</EmailLink>
      <FlexRow>
        {Object.entries(links).map(([k, v]) => (
          <a href={v.url} key={k} onMouseEnter={() => setText(`${v.name}: ${v.user}`)} onMouseLeave={clear}>
            <Icon
              src={`${process.env.PUBLIC_URL}${v.img}`}
              alt={t(`alt.${k}`)}
            />
          </a>
        ))}
      </FlexRow>
	  <User>{text}</User>
    </>
  );
};

export default NetworkLinks;
