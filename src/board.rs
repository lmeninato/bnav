use super::player::{Player, Location};
use super::tile;
use super::tile::Tile;

use serde::{Deserialize, Serialize};
use std::cmp;
use std::fs;
use std::convert::TryFrom;

// todo: add players, add ships with locations/directions
// or store info in Game struct?
#[derive(Serialize, Deserialize)]
pub struct Board {
    pub id: u32,
    pub file: &'static str,
    pub board: Box<Vec<Vec<Tile>>>,
    pub players: Box<Vec<Player>>,
    pub nrows: i16,
    pub ncols: i16,
}

impl Board {
    /// Check if tile is outside the board.
    pub fn get_tile(&self, loc: Location) -> (&Tile, Location) {
        if loc.x < 0 || loc.x >= self.nrows {
            let x = cmp::min(loc.x - self.nrows, loc.x);
            if loc.y < 0 || loc.y >= self.ncols {
                let y = cmp::min(loc.y - self.ncols, loc.y);
                return (&self.board[x as usize][y as usize], Location {x: x, y: y});
            }
            return (&self.board[x as usize][loc.y as usize], Location {x: x, y: loc.y});
        }

        if loc.y < 0 || loc.y >= self.ncols {
            let y = cmp::min(loc.y - self.ncols, loc.y);
            if loc.x < 0 || loc.x >= self.nrows {
                let x = cmp::min(loc.x - self.nrows, loc.x);
                return (&self.board[x as usize][y as usize], Location {x: x, y: y});
            }
            return (&self.board[loc.x as usize][y as usize], Location {x: loc.x, y: y});
        }

        (&self.board[loc.x as usize][loc.y as usize], Location {x: loc.x, y: loc.y})
    }
}

pub fn read_board(path: &'static str) -> Board {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    let mut board: Box<Vec<Vec<Tile>>> = Box::new(Vec::new());
    let mut players: Box<Vec<Player>> = Box::new(Vec::new());
    let ncols: i16;
    let nrows: i16;

    let player1: Player = Player::new(0, Location { x: 0, y: 0 }, tile::Direction::Up);
    players.push(player1);

    for line in contents.lines() {
        let row = line
            .split(",")
            .map(|s| tile::to_tile(&mut s.to_string()))
            .collect();

        board.push(row);
    }

    if board.len() >= 1 {
        nrows = match i16::try_from(board.len()).ok() {
            Some(val) => val,
            None => panic!("Error setting board dimensions"),
        };
        ncols = match i16::try_from(board[0].len()).ok() {
            Some(val) => val,
            None => panic!("Error setting board dimensions"),
        }
    } else {
        panic!(
            "Error: Invalid board dimensions reading board at path: {}",
            path
        );
    }

    // id is 1 for now, can use to represent unique board across many games
    Board {
        id: 1,
        file: path,
        board: board,
        players: players,
        nrows: nrows,
        ncols: ncols,
    }
}

#[test]
fn test_read_board() {
    let board = read_board("boards/test.txt");

    assert_eq!(board.board[0][0].id, String::from("s"));
    assert_eq!(board.board[0][3].id, String::from("s"));
}
