#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rand;

mod paste_id;
use paste_id::PasteID;

use std::io;
use std::path::Path;

use rocket::Data;

use std::fs::File;

#[get("/<id>")]
fn retrieve(id: PasteID) -> Option<File> {
    let filename = format!("uploads/{id}", id=id);
    File::open(&filename).ok()
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> io::Result<String> {
    let id = PasteID::new(3);
    let filename = format!("uploads/{id}", id = id);
    let host = "http://localhost";
    let url = format!("{host}/{id}\n", host=host, id=id);
    println!("save to {}", filename);
    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

        POST /
            post content

        GET /<id>
            receive content

    "
}

fn main() {
    rocket::ignite().mount("/", routes![index, upload, retrieve])
        .launch();
}
