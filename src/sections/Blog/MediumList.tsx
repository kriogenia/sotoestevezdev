import React, { useEffect, useState } from "react";
import { XMLParser } from "fast-xml-parser";
import MediumArticle, { IArticle } from "./MediumArticle";

const parser = new XMLParser();

const MediumList = () => {
  const [articles, setArticles] = useState<IArticle[]>([]);

  useEffect(() => {
    fetch(
      `https://api.allorigins.win/get?url=https://medium.com/feed/@sotoestevez`
    )
      .then((response) => response.text())
      .then((text) => parser.parse(text))
      .then((json) => json.rss.channel.item)
      .then((items) => setArticles(items));
  }, []);

  return (
    <>{articles && articles.map((article) => <MediumArticle article={article}/>)}</>
  );
};

export default MediumList;
