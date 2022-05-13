import React, { useState } from "react";
import InfiniteScroll from "react-infinite-scroll-component";
import { Introduction, Me, Portfolio, TechStack } from "../sections";
import Bottom from "../sections/Bottom";

const LOAD_TIME = 750;

const Scroll = () => {
  const [items, setItems] = useState([<Me />]);
  const [remaining, setRemaining] = useState([
    <Introduction />,
    <Portfolio />,
    <TechStack />,
  ]);
  const [hasMore, setHasMore] = useState(true);
  const [loading, setLoading] = useState(false);

  const fetchMoreData = () => {
    console.log("fetch");
    if (remaining.length === 0) {
      setHasMore(false);
      return;
    }

	if (loading) {
		return;
	}

	setLoading(true);
    setTimeout(() => {
      const [next, ...rest] = remaining;
      console.log(`Adding next component: ${items.length}`);
      setItems((state) => state.concat(next));
      setRemaining(rest);
	  setLoading(false);
    }, LOAD_TIME);
  };

  return (
    <InfiniteScroll
      dataLength={items.length}
      next={fetchMoreData}
      hasMore={hasMore}
      loader={<h4>Loading...</h4>} /// TODO replace with spinner
      endMessage={<Bottom />}
    >
      {items.map((i, index) => (
        <div key={index}>{i}</div>
      ))}
    </InfiniteScroll>
  );
};

export default Scroll;
