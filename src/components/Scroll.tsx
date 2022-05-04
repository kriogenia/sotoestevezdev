import React, { useState } from "react";
import InfiniteScroll from "react-infinite-scroll-component";
import ThemeSwitch from "./ThemeSwitch";
import { Intro, Portfolio } from "../sections";
import TechStack from "../sections/TechStack";

const Scroll = () => {
  const [state, setState] = useState({ items: [<Intro />] });
  const [extraComponents, setRemaining] = useState([
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
      setState({
        items: state.items.concat(next),
      });
      setRemaining(remaining);
    }, 500);
  };

  return (
    <div>
      <h1>SotoEstevez.dev</h1>
	  <ThemeSwitch/>
      <InfiniteScroll
        dataLength={state.items.length}
        next={fetchMoreData}
        hasMore={hasMore}
        loader={<h4>Loading...</h4>} /// TODO replace with spinner
        endMessage={
          /// TODO replace end
          <p style={{ textAlign: "center" }}>
            <b>This is the end</b>
          </p>
        }
      >
        {state.items.map((i) => i)}
      </InfiniteScroll>
    </div>
  );
};

export default Scroll;
