pub mod board;
pub mod config;
mod game;
pub mod player;
mod tile;

// use crate::tile as tile;
// use crate::config as bnav_config;

#[cfg(test)]
mod tests {
    #[test]
    fn test_config() {
        let y = crate::config::FRONTEND_PATH;

        assert_eq!(y, y);
    }
}
