import React from 'react';

import Tile from "./Tile";

class Board extends React.Component {
    constructor(props) {
        super(props);

        this.state = { 
            isLoaded: false,
            board: null
        };

        fetch("http://127.0.0.1:8080/api/board", {
                            crossdomain: true })
                         .then(response => response.json())
                         .then(data => data['board'])
                         .then(board => this.setBoard(board))
                         .catch(function(error){
                             console.log("Error fetching api/board")
                             console.log(error.request);
                             console.log(error.message);
                         });
    }

    setBoard(board) {
        console.log("Setting board with data from API")
        board = board.map(row => row.map(tile => tile['id']))
        this.setState({
            ...this.state,
            tiles: board,
            isLoaded: true
        })
    }

    renderRow(row, col_index) {
        return (
            <div className="board-row" key={col_index}>
            {
                row.map((tile_id, row_index) => <Tile tile_id={tile_id} row_index={row_index} col_index={col_index}/>)
            }
            </div>
        )
    }

    render() {
        const { isLoaded, tiles } = this.state;
        
        if (!isLoaded){
            return (
                <div className="Board"></div>
            )
        }

        return (
            <div className="Board">
            {
                tiles.map((boardRow, i) => this.renderRow(boardRow, i))
            }            
            </div>
        )
    }
}

export default Board;