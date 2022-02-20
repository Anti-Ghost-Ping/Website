#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use rusqlite::Connection;

mod info;
// mod api;
mod structs;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(info::stage())
        .mount("/static", FileServer::from(relative!("static")))
}