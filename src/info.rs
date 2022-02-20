use std::collections::HashMap;

use rocket::{response::Redirect, fairing::AdHoc};
use rocket_dyn_templates::Template;

#[get("/")]
pub fn index() -> Template {
    let context: HashMap<&str, u64> = [("ghost-pings", 10_000), ("server-count", 1_000_000)]
    .iter().cloned().collect();

    Template::render("index", &context)
}

#[get("/invite")]
pub fn invite() -> Redirect {
    Redirect::to("https://discord.com/oauth2/authorize?client_id=699522828147490826&permissions=911488&scope=bot")
}

#[get("/discord")]
pub fn discord() -> Redirect {
    Redirect::to("https://discord.com/invite/WpHjk8Z")
}

#[get("/source")]
pub fn source() -> Redirect {
    Redirect::to("https://github.com/Anti-Ghost-Ping/Anti-Ghost-Ping")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Info stage", |rocket| async {
        rocket.mount("/", routes![index, invite, discord, source])
    })
}