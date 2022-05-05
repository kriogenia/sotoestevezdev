import React from "react";
import i18n from "../i18n";

const changeLanguage = () => {
  const language = i18n.language === "en" ? "gl" : "en";
  i18n.changeLanguage(language);
};

const LanguageSelector = () => {
  return (
    <p>
      <button onClick={changeLanguage}>Language</button>
    </p>
  );
};

export default LanguageSelector;
