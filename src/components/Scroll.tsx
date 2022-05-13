import React, { useState } from "react";
import InfiniteScroll from "react-infinite-scroll-component";
import { Introduction, Me, Portfolio, TechStack } from "../sections";
import Bottom from "../sections/Bottom";

const LOAD_TIME = 500;

const Scroll = () => {
  const [state, setState] = useState({ items: [<Me />] });
  const [extraComponents, setRemaining] = useState([
    <Introduction />,
    <Portfolio />,
    <TechStack />,
  ]);
  const [hasMore, setHasMore] = useState(true);

  const fetchMoreData = () => {
    if (extraComponents.length === 0) {
      setHasMore(false);
      return;
    }

    setTimeout(() => {
      const [next, ...remaining] = extraComponents;
	  console.log(`Adding next component: ${state.items.length}`);
      setState({
        items: state.items.concat(next),
      });
      setRemaining(remaining);
    }, LOAD_TIME);
  };

  return (
    <InfiniteScroll
      dataLength={state.items.length}
      next={fetchMoreData}
      hasMore={hasMore}
      loader={<h4>Loading...</h4>} /// TODO replace with spinner
      endMessage={<Bottom />}
    >
      {state.items.map((i) => i)}
    </InfiniteScroll>
  );
};

export default Scroll;
