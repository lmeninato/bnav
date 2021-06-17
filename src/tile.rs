use std::fmt;

pub struct WhirlTile {
    clock_wise: bool,
    top: bool,
    left: bool
}

pub enum WindTile {
    Up,
    Down, 
    Left,
    Right
}

pub enum TileType {
    Whirl(WhirlTile),
    Rock,
    Wind(WindTile),
    Sea
}

pub struct Tile {
    tile: TileType,
    id: String
}

fn get_whirl(clock_wise: bool, top: bool, left: bool, s: &String) -> Tile {
    Tile {tile: TileType::Whirl(WhirlTile {clock_wise: clock_wise, top: top, left: left}), id: s.clone()}
}

fn to_tile(s: &mut String) -> Tile {
    s.retain(|c| !c.is_whitespace());
    let first = &s[0..0];
    let mut res = Tile {tile: TileType::Sea, id: s.clone()};
    for &_input in &["s", "r", "w", "c", "x"] {
        res = match first {
            "s" => Tile {tile: TileType::Sea, id: s.clone()}, // sea tile
            "r" => Tile {tile: TileType::Rock, id: s.clone()}, // rock tile
            "w" => match &s[1..1] {                            // wind tile
                "u" => Tile {tile: TileType::Wind(WindTile::Up), id: s.clone()}, 
                "d" => Tile {tile: TileType::Wind(WindTile::Down), id: s.clone()}, 
                "l" => Tile {tile: TileType::Wind(WindTile::Left), id: s.clone()}, 
                "r" => Tile {tile: TileType::Wind(WindTile::Right), id: s.clone()},
                _ => panic!("should not reach")
            }, 
            "c" => match &s[2..3] {                           
                "tl" => get_whirl(true, true, true, s),
                "tr" => get_whirl(true, true, false, s),
                "bl" => get_whirl(true, false, true, s),
                "br" => get_whirl(true, false, false, s),
                _ => panic!("should not reach")
            }, 
            "x" => match &s[2..3] {                           
                "tl" => get_whirl(false, true, true, s),
                "tr" => get_whirl(false, true, false, s),
                "bl" => get_whirl(false, false, true, s),
                "br" => get_whirl(false, false, false, s),
                _ => panic!("should not reach")
            },
             
            _ => Tile {tile: TileType::Sea, id: s.clone()}
        };
    }

    res
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.tile {
            TileType::Whirl(_) => write!(f, "Whirl tile with ID: {}", self.id),
            TileType::Rock => write!(f, "Rock tile with ID: {}", self.id),
            TileType::Wind(_) => write!(f, "Wind tile with ID: {}", self.id),
            TileType::Sea => write!(f, "Sea tile with ID: {}", self.id)
        }
    }
}

#[test]
fn test() {
    let x = Tile {tile: TileType::Sea, id: String::from("s")};

    assert_eq!(x.id, String::from("s"));
}
