import React from "react";

import Tile from "./Tile";

class Board extends React.Component {
  constructor(props) {
    super(props);

    this.state = {
      isLoaded: false,
      board: null,
      players: null
    };

  }

  componentDidMount(){
    fetch("http://127.0.0.1:8080/api/board", {
      crossdomain: true,
    })
      .then((response) => response.json())
      .then((data) => ({board: data["board"], players: data["players"]}))
      .then((data) => {this.setBoard(data["board"]); this.setPlayers(data["players"])})
      .then(() => this.setState({...this.state, isLoaded: true}))
      .catch(function (error) {
        console.log("Error fetching api/board ... is the server up?");
        console.log(error.request);
        console.log(error.message);
    });
  }

  setBoard(board) {
    console.log("Setting board with data from API");
    board = board.map((row) => row.map((tile) => tile["id"]));
    this.setState({
      ...this.state,
      tiles: board,
    });
  }

  setPlayers(players) {
    console.log("Placing players on board with data from API");    

    this.setState({
        ...this.state,
        players: players,
      });
  }

  isPlayerAtLocation(row_index, col_index) {
      for (let i=0; i<this.state.players.length; i++){
        let player = this.state.players[i];
        if (player["loc_x"] == row_index && player["loc_y"] == col_index){
            return player;
        }
      }
 
      return null;
  }

  renderTile(tile_id, row_i, col_i) {
    const player = this.isPlayerAtLocation(row_i, col_i);
    if (player != null){
        console.log(player['direction']);
        return (<Tile tile_id={tile_id}
                      row_index={row_i} 
                      col_index={col_i} 
                      key={row_i + '' + col_i}
                      ship={player["direction"]}
                      />);
    } 

    return (<Tile tile_id={tile_id}
                  row_index={row_i} 
                  col_index={col_i} 
                  key={row_i + '' + col_i}
                  />);
  }

  renderRow(row, col_index) {
    return (
      <div className="board-row" key={col_index}>
        {row.map((tile_id, row_index) =>  this.renderTile(tile_id, row_index, col_index))}
      </div>
    );
  }

  render() {
    const { isLoaded, tiles } = this.state;

    if (!isLoaded) {
      return <div className="BoardLoading">Loading...</div>;
    }

    return (
      <div className="Board">
        {tiles.map((boardRow, i) => this.renderRow(boardRow, i))}
      </div>
    );
  }
}

export default Board;
