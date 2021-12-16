#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate reqwest;

use std::collections::HashMap;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use rocket::response::Redirect;
use serde::Deserialize;

#[derive(Deserialize)]
struct Stats {
    server_count: u64,
    total_pings: u64
}

fn get_stats() -> Result<HashMap<&'static str, u64>, Box<dyn std::error::Error>>{
    // let data: Stats = reqwest::blocking::get("https://api.ghostping.xyz/v1/stats/")?.json()?;

    // let context: HashMap<&str, u64> = [("ghost-pings", data.total_pings), ("server-count", data.server_count)]
    //     .iter().cloned().collect();

    let context: HashMap<&str, u64> = [("ghost-pings", 10_000), ("server-count", 1_000_000)]
        .iter().cloned().collect();
    Ok(context)
}

#[get("/")]
fn index() -> Template {
    let context = get_stats().unwrap();

    Template::render("index", &context)
}

#[get("/invite")]
fn invite() -> Redirect {
    Redirect::to("https://discord.com/oauth2/authorize?client_id=699522828147490826&permissions=911488&scope=bot")
}

#[get("/discord")]
fn discord() -> Redirect {
    Redirect::to("https://discord.com/invite/WpHjk8Z")
}

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, invite, discord])
        .launch();
}