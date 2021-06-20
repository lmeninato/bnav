use super::tile::Direction;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub health: u32,
    pub dmg_taken: u32,
    pub dmg: u32,
    pub loc_x: u32,
    pub loc_y: u32,
    pub direction: Direction,
}

impl Player {
    pub fn new(id: u32, loc_x: u32, loc_y: u32, direction: Direction) -> Player {
        Player {
            id: id, 
            health: 10, 
            dmg_taken: 0, 
            dmg: 1, 
            loc_x: loc_x,
            loc_y: loc_y,
            direction: direction
        }
    }
}

