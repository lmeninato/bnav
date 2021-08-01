#![feature(proc_macro_hygiene, decl_macro)]

use bnav::board;
use bnav::config as bnav_config;

use warp::Filter;

#[tokio::main]
async fn main() {

    // GET /
    let react_app = warp::fs::dir(bnav_config::FRONTEND_PATH)
        .with(warp::log("test"));

    // GET /api/board
    let board = warp::path!("api" / "board")
        .map(|| {
            let board = board::read_board("boards/test.txt");
            warp::reply::json(&board)
        });

    let routes = react_app
        .or(board);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
} 
