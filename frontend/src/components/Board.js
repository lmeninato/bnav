import React from 'react';

import Tile, {TileTypes} from "./Tile";


class Board extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            tiles: Array(props.rows).fill(Array(props.cols).fill(TileTypes['SEA']))
        };
    }

    renderRow(row) {
        return (
            <div className="board-row">
            {
                row.map((tile) => <Tile tileType={tile}/>)
            }
            </div>
        )
    }

    render() {
        return (
            <div className="Board">
            {
                this.state.tiles.map((boardRow) => this.renderRow(boardRow))
            }            
            </div>
        )
    }
}

export default Board;