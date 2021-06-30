use super::tile::Direction;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Location {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
pub struct LocDir {
    pub loc: Location,
    pub dir: Direction,
}

#[derive(Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Player {
    pub id: usize,
    pub health: u32,
    pub dmg_taken: u32,
    pub dmg: u32,
    pub location: Location,
    pub direction: Direction,
}

impl Player {
    pub fn new(id: usize, location: Location, direction: Direction) -> Player {
        Player {
            id: id,
            health: 10,
            dmg_taken: 0,
            dmg: 1,
            location: location,
            direction: direction,
        }
    }
}
