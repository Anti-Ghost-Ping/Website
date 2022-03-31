use rocket::{fairing::AdHoc, response::Redirect, routes};

#[get("/invite")]
pub async fn invite() -> Redirect {
    Redirect::to("https://discord.com/api/oauth2/authorize?client_id=699522828147490826&permissions=274878187648&scope=bot%20applications.commands")
}

#[get("/discord")]
pub async fn discord() -> Redirect {
    Redirect::to("https://discord.com/invite/WpHjk8Z")
}

#[get("/source")]
pub async fn source() -> Redirect {
    Redirect::to("https://github.com/Anti-Ghost-Ping/Anti-Ghost-Ping")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Info stage", |rocket| async {
        rocket.mount("/", routes![invite, discord, source])
    })
}
