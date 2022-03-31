#[macro_use]
extern crate rocket;

use api::Stats;
use rocket::fs::{relative, FileServer};
use rocket::{routes, State};
use rocket_dyn_templates::Template;
use std::collections::HashMap;
use std::fs;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::Duration;

mod api;
mod info;

#[get("/")]
pub async fn index(stats: &State<Arc<Stats>>) -> Template {
    let context: HashMap<&str, u32> = HashMap::from([
        ("ghost-pings", stats.total_pings.load(Relaxed)),
        ("server-count", stats.guild_count.load(Relaxed)),
    ]);

    Template::render("index", &context)
}

#[launch]
async fn rocket() -> _ {
    let file = fs::read_to_string("stats.json").expect("Unable to read file");
    let stats: Arc<Stats> = Arc::new(serde_json::from_str(&file).expect("Could not parse json"));
    println!("{:#?}", stats);
    dotenv::dotenv().ok();
    rocket::build()
        .attach(Template::fairing())
        .attach(info::stage())
        .attach(api::stage())
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .manage(stats.clone())
        .attach(rocket::fairing::AdHoc::on_liftoff(
            "Background task",
            |_| {
                Box::pin(async move {
                    rocket::tokio::spawn(async move {
                        let mut interval = rocket::tokio::time::interval(Duration::from_secs(10));
                        interval.tick().await;
                        loop {
                            interval.tick().await;
                            let json = serde_json::to_string(&stats).unwrap();
                            rocket::tokio::fs::write("stats.json", json).await.unwrap();
                        }
                    });
                })
            },
        ))
}
