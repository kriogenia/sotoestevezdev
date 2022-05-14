import React, { ChangeEvent, useState } from "react";
import styled from "styled-components";
import i18n from "../../i18n";
import { backgroundColor, primaryColor } from "../../theme/colors";
import { topSize } from "../../theme/styles";
import langs from "./langs.json";

const Dropdown = styled.select`
	height: ${topSize};
	background-color: ${backgroundColor};
	color: ${primaryColor};
	border: 2px solid ${primaryColor};
	border-radius: 5px;
	font-size: 100%;
`

const LanguageSelector = () => {
  const [selected, setSelected] = useState("en");

  const changeLanguage = ({
    target: { value },
  }: ChangeEvent<HTMLSelectElement>) => {
    setSelected(value);
    i18n.changeLanguage(value);
  };

  return (
    <Dropdown value={selected} onChange={changeLanguage}>
      {langs.langs.map((lang) => (
        <option key={lang} value={lang}>
          {lang}
        </option>
      ))}
    </Dropdown>
  );
};

export default LanguageSelector;
