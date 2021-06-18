use super::tile::Tile;
use super::tile;

use std::fs;
use serde::{Deserialize, Serialize};

// todo: add players, add ships with locations/directions
// or store info in Game struct?
#[derive(Serialize, Deserialize)]
pub struct Board<'a> {
    pub id: u32,
    pub file: &'a str,
    pub board: Box<Vec<Vec<Tile>>>
}

// impl Board {
//     pub fn as_json(&self) -> content::Json<&str> {
//         let res = String::from("");
//         for row in self.board {
//             let row.map(|tile| format!("\"{}\"", tile.id))
//                .join(",")
//         }
//         board = self.board.join(",")
//     }
// }

pub fn read_board(path: &str) -> Board {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let mut board: Box<Vec<Vec<Tile>>> = Box::new(Vec::new());

    for line in contents.lines() {
        let row = line.split(",")
            .map(|s| tile::to_tile(&mut s.to_string()))
            .collect();

        board.push(row);
    }

    // id is 1 for now, can use to represent unique board across many games
    Board {id: 1, file: path, board: board}
}

#[test]
fn test_read_board() {
    let board = read_board("boards/test.txt");

    assert_eq!(board.board[0][0].id, String::from("s"));
    assert_eq!(board.board[0][4].id, String::from("cwtr"));
}
