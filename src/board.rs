use super::tile::Tile;
use super::tile;

use std::fs;

pub fn read_board(path: &str) -> Box<Vec<Vec<String>>>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let mut res: Box<Vec<Vec<String>>> = Box::new(Vec::new());

    for line in contents.lines() {
        let row = line.split(",")
            .map(|s| s.to_string())
            .collect();

        res.push(row);
    }
    
    res
}

#[test]
fn test_read_board() {
    let board = read_board("boards/test.txt");
    assert_eq!(board[0][0], String::from("s"));
    assert_eq!(board[0][4], String::from("trcw"));
}
