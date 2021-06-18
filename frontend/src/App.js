import React from "react";

import Board from "./components/Board";

import './App.scss';

function App() {
  return (
    <div className="App" style = {{textAlign: "center"}}>
      <h1 className="start-text">hello world</h1>
      <Board />
    </div>
  );
}

export default App;