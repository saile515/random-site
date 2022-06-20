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
			<button onClick={handleClick}>Click Here</button>
		</div>
	);
}

export default App;
