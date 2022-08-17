import { FC } from "react";
import styled from "styled-components";
//import { backgroundColor, tagColor } from "../../theme/colors";
import { BookIcon } from "../../theme/icons";
import { Link } from "../../theme/styles";

const Container = styled.article`
  width: 60%;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: start;
  gap: 25px;
  margin: 15px;

  @media (hover: none) {
    width: 100%;
  }
`;

const ReadIcon = styled(BookIcon)`
  height: 5rem;
`;

const Article = styled.article`
  text-align: left;
`;

const Title = styled.h2`
  margin-bottom: 0.5rem;
  font-size: 2rem;
  line-height: 1.2;
`;

const DateTag = styled.p`
  font-size: 0.9rem;
`;
/*
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
*/
export interface IArticle {
  title: string;
  link: string;
  category: string[];
  pubDate: string;
}

interface Props {
  article: IArticle;
}

const MediumArticle: FC<Props> = ({ article }) => {
  return (
    <Container>
      <Link href={article.link}>
        <ReadIcon />
      </Link>
      <Article>
        <Title>{article.title}</Title>
		<DateTag>{new Date(article.pubDate).toDateString()}</DateTag>
        {/*article.category.map((cat) => (
          <Tag>{cat}</Tag>
		))*/}
      </Article>
    </Container>
  );
};

export default MediumArticle;
