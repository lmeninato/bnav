import React from "react";

import Board from "./components/Board";
import Player from "./components/Player";

import "./App.scss";

function App() {
  return (
    <div className="App" style={{ textAlign: "center" }}>
      <h1 className="start-text">Welcome to Battle Navigation</h1>
      <Board />
      <Player startX={0} startY={0} direction={"RIGHT"} />
      <Player startX={0} startY={0} direction={"LEFT"} />
    </div>
  );
}

export default App;
