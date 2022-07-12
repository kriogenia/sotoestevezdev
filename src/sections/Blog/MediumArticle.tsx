import { FC } from "react";
import styled from "styled-components";
import { LinearDivider } from "../../components/Divider";
import { backgroundColor, frontColor, tagColor } from "../../theme/colors";
import { Link } from "../../theme/styles";

const Container = styled.article`
  width: 40%;
  text-align: left;
  margin: 15px 0px;

  :hover {
    background: ;
  }

  @media (hover: none) {
    width: 100%;
  }
`;

const Title = styled.h2`
  margin-bottom: 0.5rem;
  font-size: 2rem;
  line-height: 1.2;
`;

const Subtitle = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: start;
  gap: 5px;
`;

const DateTag = styled.span`
  font-size: 0.8rem;
`;

const Dot = styled(LinearDivider)`
  fill: ${frontColor};
  height: 5px;
`;

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
      <Title>{article.title}</Title>
      <Subtitle>
        <DateTag>{new Date(article.pubDate).toDateString()}</DateTag>
        <Dot />
        <Link href={article.link}>Read</Link>
      </Subtitle>
      {article.category.map((cat) => (
        <Tag>{cat}</Tag>
      ))}
    </Container>
  );
};

export default MediumArticle;
