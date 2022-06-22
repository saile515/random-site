import "./App.css";

import React from "react";

function App() {
	function handleClick() {
		fetch(process.env.REACT_APP_SERVER_URL + "/domain")
			.then((res) => res.text())
			.then((data) => {
				console.log(data);
				window.open("http://" + data, "_blank", "noreferrer");
			});
	}

	return (
		<div className="App">
			<h1>Random Site Finder</h1>
			<button className="mainButton" onClick={handleClick}>
				Click Here
			</button>
			<p>
				<b>Warning!</b> The content on the sites is not moderated. There is a high possibility of coming across NSFW content.
			</p>
		</div>
	);
}

export default App;
