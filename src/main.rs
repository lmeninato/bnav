#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket; 

use std::io;
use std::env; 
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

// use crate::tile as tile;
use bnav::config as bnav_config;

#[get("/")]
fn index() -> io::Result<NamedFile> {
  files(Path::new("index.html").to_path_buf())
}

#[get("/<file..>")]
fn files(file: PathBuf) -> io::Result<NamedFile> {
    let page_directory_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), bnav_config::FRONTEND_PATH);
    NamedFile::open(Path::new(&page_directory_path).join(file))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .launch();
}
