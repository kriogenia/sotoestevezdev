import React, { useState } from "react";
import InfiniteScroll from "react-infinite-scroll-component";
import "./App.css";
import { Intro, Portfolio } from "./sections";

const App = () => {
	const [state, setState] = useState({ items: [<Intro />] });
	const [extraComponents, setRemaining] = useState([<Portfolio />]);
	const [hasMore, setHasMore] = useState(true);

	const fetchMoreData = () => {
		console.log("fect");
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
			<InfiniteScroll
				dataLength={state.items.length}
				next={fetchMoreData}
				hasMore={hasMore}
				loader={<h4>Loading...</h4>}			/// TODO replace with spinner
				endMessage={
					<p style={{ textAlign: "center" }}>
						<b>Yay! You have seen it all</b>
					</p>
				}
			>
				{state.items.map((i) => i)}
			</InfiniteScroll>
		</div>
	);
};

export default App;
