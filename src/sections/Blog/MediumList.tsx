import { useEffect, useState } from "react";
import { XMLParser } from "fast-xml-parser";
import MediumArticle, { IArticle } from "./MediumArticle";
import Fallback from "../../components/Fallback";

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
    <>
      {articles.length > 0 ? (
        articles.map((article) => <MediumArticle article={article} />)
      ) : (
        <Fallback />
      )}
    </>
  );
};

export default MediumList;
