import React from "react";

import Board from "./Board";

class Game extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            isLoaded: false,
            players: [],
            board: null,
        };
    }

    componentDidMount() {
        fetch()
    }

    render() {
        return (
            <Board />
          );
    }
}

export default Game;