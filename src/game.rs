use super::board::{read_board, Board};
use super::player::{Location, LocDir, Player};
use super::tile::{TileType, Direction};
use std::collections::HashMap;

pub struct Game {
    pub id: u32,
    pub board: Board,
    next_locs: HashMap<Player, Vec<LocDir>>,
}

#[derive(Debug, Copy, Clone)]
pub enum Move {
    Forward,
    Left,
    Right,
    None,
}

pub enum Attack {
    Grapple,
    Shoot,
}

pub enum AttackMove {
    LeftAttackMove,
    RightAttackMove,
}

pub struct Turn {
    pub player: Player,
    pub moves: Vec<Move>,
    pub attack_moves: Vec<AttackMove>,
}

impl Game {
    pub fn new(path: &'static str) -> Game {
        Game {
            id: 1, // just 1 for now, can be used to keep track of all unique games later.
            board: read_board(path),
            next_locs: HashMap::new(),
        }
    }

    fn save_next_loc(&mut self, player: Player, player_move: Move) {
        let next_locs_with_dirs = self.get_visited_locations(player, player_move);
        self.next_locs.insert(player, next_locs_with_dirs);
    }

    fn update_player_locs(&mut self, new_player_locs: &HashMap<Player, LocDir>) {
        for (p, loc) in new_player_locs {
            self.board.players[p.id].location = loc.loc;
        }
    }

    // maybe should be f: Vec<Turn> -> None?
    pub fn get_next_state(&mut self, moves: Vec<Move>, _attack_moves: Vec<AttackMove>) {
        let mut next_player_locations: HashMap<Player, LocDir> = HashMap::new();
        for i in 0..self.board.players.len() {
            let player = self.board.players[i];
            let player_move = moves[i];
            // can easily lookup collisions
            self.save_next_loc(player, player_move);

            // lets just handle moving a single player for now
            let next_loc_dir = self.handle_move(player, player_move);
            next_player_locations.insert(player, next_loc_dir);
        }

        self.update_player_locs(&next_player_locations);

        // handle whirls/whinds
        // handle attack_moves
    }

    fn handle_move(&self, player: Player, player_move: Move) -> LocDir {
        match player_move {
            Move::Forward => self.handle_forward_collision(player, player_move),
            Move::Left => self.handle_forward_collision(player, player_move),
            Move::Right => self.handle_forward_collision(player, player_move),
            _ => LocDir {loc: player.location, dir: player.direction},
        }
    }

    fn handle_forward_collision(&self, player: Player, _player_move: Move) -> LocDir {
        let next_locdir = match self.next_locs.get(&player) {
            Some(next_locdir) => next_locdir[0],
            None => panic!("Error accessing map of next player moves for forward move.")
        };
        let loc_with_dir = LocDir {loc: player.location, dir: player.direction};

        let (tile, loc) = self.board.get_tile(next_locdir.loc);

        match tile.tile {
            TileType::Rock => return loc_with_dir,
            _ => (),
        };

        // outside board
        if loc.x < 0 || loc.x >= self.board.ncols || loc.y < 0 || loc.y >= self.board.nrows {
            return loc_with_dir;
        }

        for (_, locs) in &self.next_locs {
            if next_locdir == locs[0] {
                return loc_with_dir;
            }
        }

        // no collision detected
        next_locdir
    }

    fn get_visited_locations(&self, player: Player, player_move: Move) -> Vec<LocDir> {
        let mut res: Vec<LocDir> = Vec::new();

        let mut add_location = |diff_x: i16, diff_y: i16| {
            let translated_locdir = translate_location(
                player.location,
                Location {
                    x: player.location.x + diff_x,
                    y: player.location.y + diff_y,
                },
                player.direction,
                player_move
            );
            res.push(translated_locdir);
        };

        add_location(0, -1);

        match player_move {
            Move::Left => add_location(-1, -1),
            Move::Right => add_location(1, -1),
            _ => (),
        };

        res
    }
}

/// If collision occurs return (x_loc, y_loc, Direction) of player
/// after collision, otherwise return None.
///
/// be clever about what algebra on move + orientation (direction) means?
///
/// (m+1) x (n+1) board
/// (0, 0) (0, 1) ... (0, n)
/// (1, 0) (1, 1) ... (1, n)
/// ...  
/// (m, 0) ....       (m, n)
///
/// enumerating all the cases...
///
/// Direction UP:
/// forward is (-1, 0)
/// left is (-1, 0) + UP->LEFT + (0, -1)
/// right is (-1, 0) + UP->RIGHT + (0, 1)
///
/// Direction LEFT:
/// forward is (0, -1)
/// left is (0, -1) + LEFT->DOWN + (1, 0)
/// right is (0, -1) + LEFT->UP + (-1, 0)
///
/// Direction DOWN:
/// forward is (1, 0)
/// left is (1, 0) + DOWN->RIGHT + (0, 1)
/// right is (1, 0) + DOWN->LEFT + (0, -1)
///
/// Direction RIGHT:
/// forward is (0, 1)
/// left is (0, 1) + RIGHT->UP + (-1, 0)
/// right is (0, 1) + RIGHT->DOWN + (1, 0)
/// 
/// Rotate 90 degrees counter-clockwise around point (i,j) in 2d plane:
/// Rotation Matrix R is:
/// |0 -1|
/// |1  0|
/// thus L = R *[x-i, y-j] + [i, j]
fn translate_location(
    start_loc: Location,
    end_loc: Location,
    dir: Direction,
    player_move: Move,
) -> LocDir {
    let rotate = |end_loc: Location| -> Location {
        Location {
            x: start_loc.x + start_loc.y - end_loc.y,
            y: end_loc.x + start_loc.y - start_loc.x,
        }
    };

    let next_loc = match dir {
        Direction::Up => end_loc,
        Direction::Left => rotate(end_loc),
        Direction::Down => rotate(rotate(end_loc)),
        Direction::Right => rotate(rotate(rotate(end_loc))),
    };

    LocDir {loc: next_loc, dir: translate_direction(dir, player_move)}
}
  
fn translate_direction(dir: Direction, player_move: Move) -> Direction {
    let dir_to_int = |dir: Direction| -> i8 {
        match dir {
            Direction::Up => 0,
            Direction::Left => 1,
            Direction::Down => 2,
            Direction::Right => 3,
        }
    };

    let new_dir_as_int = match player_move {
        Move::Left => (dir_to_int(dir) + 1) % 4,
        Move::Right => (dir_to_int(dir) + 3) % 4,
        _ => dir_to_int(dir),
    };

    match new_dir_as_int {
        0 => Direction::Up,
        1 => Direction::Left,
        2 => Direction::Down,
        3 => Direction::Right,
        _ => panic!("Should not be reached."),
    }
}
