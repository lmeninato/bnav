use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct WhirlTile {
    pub clock_wise: bool,
    pub top: bool,
    pub left: bool,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum TileType {
    Whirl(WhirlTile),
    Rock,
    Wind(Direction),
    Sea,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Tile {
    pub tile: TileType,
    pub id: String,
    pub direction: Option<Direction>,
}

pub trait GetDirection {
    fn get_direction(&self) -> Option<Direction>;
}

impl GetDirection for WhirlTile {
    fn get_direction(&self) -> Option<Direction> {
        match self {
            WhirlTile {
                clock_wise: true,
                top: true,
                left: true,
            } => Some(Direction::Right),
            WhirlTile {
                clock_wise: true,
                top: true,
                left: false,
            } => Some(Direction::Down),
            WhirlTile {
                clock_wise: true,
                top: false,
                left: true,
            } => Some(Direction::Up),
            WhirlTile {
                clock_wise: true,
                top: false,
                left: false,
            } => Some(Direction::Left),
            WhirlTile {
                clock_wise: false,
                top: true,
                left: true,
            } => Some(Direction::Down),
            WhirlTile {
                clock_wise: false,
                top: true,
                left: false,
            } => Some(Direction::Left),
            WhirlTile {
                clock_wise: false,
                top: false,
                left: true,
            } => Some(Direction::Right),
            WhirlTile {
                clock_wise: false,
                top: false,
                left: false,
            } => Some(Direction::Up),
        }
    }
}

fn get_whirl(clock_wise: bool, top: bool, left: bool, s: &String) -> Tile {
    let whirl = WhirlTile {
        clock_wise: clock_wise,
        top: top,
        left: left,
    };
    let tile = TileType::Whirl(whirl);

    Tile {
        tile: tile,
        id: s.clone(),
        direction: whirl.get_direction(),
    }
}

pub fn to_tile(s: &mut String) -> Tile {
    s.retain(|c| !c.is_whitespace());
    let first = &s[..1];

    match first {
        "s" => Tile {tile: TileType::Sea, id: s.clone(), direction: None}, // sea tile
        "r" => Tile {tile: TileType::Rock, id: s.clone(), direction: None}, // rock tile
        "w" => match &s[1..2] { // wind tile
            "u" => Tile {tile: TileType::Wind(Direction::Up), id: s.clone(), direction: Some(Direction::Up)}, 
            "d" => Tile {tile: TileType::Wind(Direction::Down), id: s.clone(), direction: Some(Direction::Down)}, 
            "l" => Tile {tile: TileType::Wind(Direction::Left), id: s.clone(), direction: Some(Direction::Left)}, 
            "r" => Tile {tile: TileType::Wind(Direction::Right), id: s.clone(), direction: Some(Direction::Right)},
            _ => panic!("should not reach: error parsing wind tile: s: {}, direction: {}", s, &s[1..1])
        },
        "c" => match &s[2..] { // clockwise whirl tile                    
            "tl" => get_whirl(true, true, true, s),
            "tr" => get_whirl(true, true, false, s),
            "bl" => get_whirl(true, false, true, s),
            "br" => get_whirl(true, false, false, s),
            _ => panic!("should not reach: error parsing clockwise whirl tile: s: {}, direction: {}", s, &s[2..])
        },
        "x" => match &s[2..] { // counter-clockwise whirl tile               
            "tl" => get_whirl(false, true, true, s),
            "tr" => get_whirl(false, true, false, s),
            "bl" => get_whirl(false, false, true, s),
            "br" => get_whirl(false, false, false, s),
            _ => panic!("should not reach: error parsing counter-clockwise whirl tile: s: {}, direction: {}", s, &s[2..])
        },
        _ => {
            panic!("Warning, did not match tile string input: {}", s);
            // Tile {tile: TileType::Sea, id: s.clone(), direction: Direction::None}
        }
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.tile {
            TileType::Whirl(_) => write!(f, "Whirl tile with ID: {}", self.id),
            TileType::Rock => write!(f, "Rock tile with ID: {}", self.id),
            TileType::Wind(_) => write!(f, "Wind tile with ID: {}", self.id),
            TileType::Sea => write!(f, "Sea tile with ID: {}", self.id),
        }
    }
}

#[test]
fn test_tiles() {
    let dir = Direction::Up;

    let x = Tile {
        tile: TileType::Sea,
        id: String::from("s"),
        direction: Copy(dir),
    };

    assert_eq!(x.id, String::from("s"));
    assert_eq!(x.direction, dir);

    let mut tile_id = String::from("cwtl");
    let tile = to_tile(&mut tile_id);

    assert_eq!(tile.id, String::from("cwtl"));
    assert_eq!(tile.direction, Direction::Right)
}
