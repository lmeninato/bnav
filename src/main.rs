#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::NamedFile;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
// use rocket::response::content;
use rocket_contrib::json::Json;

// use crate::tile as tile;
use bnav::board;
use bnav::board::Board;
use bnav::config as bnav_config;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    files(Path::new("index.html").to_path_buf())
}

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        bnav_config::FRONTEND_PATH
    );
    NamedFile::open(Path::new(&page_directory_path).join(file))
}

#[get("/api/board")]
fn board() -> Json<Board<'static>> {
    let board = board::read_board("boards/test.txt");
    Json(board)
}

// #[post("/api/moves/<player_id>")]
// fn get_moves(player_id: u32) {

// }

fn main() {
    rocket::ignite()
        .attach(CORS)
        .mount("/", routes![index, files, board])
        .launch();
}
