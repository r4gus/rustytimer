#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/")]
fn index() -> std::io::Result<NamedFile> {
    rocket::response::NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, file])
        .mount("/", StaticFiles::from("static"))
        .launch();
}