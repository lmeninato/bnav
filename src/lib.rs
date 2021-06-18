pub mod config;

pub mod board;
mod game;
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
