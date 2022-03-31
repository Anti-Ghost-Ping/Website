use std::env;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, State};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Stats {
    pub guild_count: AtomicU32,
    pub total_pings: AtomicU32,
    pub ppm: AtomicU32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostData {
    pub guild_count: u32,
    pub total_pings: u32,
}

pub struct ApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let key = env::var("API_KEY").unwrap();
        if let Some(header) = req.headers().get_one("API_KEY") {
            if header == key {
                return Outcome::Success(ApiKey);
            } else {
                return Outcome::Failure((Status::Unauthorized, ()));
            }
        } else {
            return Outcome::Failure((Status::Unauthorized, ()));
        }
    }
}

#[get("/stats")]
pub async fn get_stats(stats: &State<Arc<Stats>>) -> Option<Value> {
    Some(json!({
        "guild_count": stats.guild_count,
        "total_pings": stats.total_pings,
        "ppm": stats.ppm
    }))
}

#[post("/stats", format = "json", data = "<post>")]
pub async fn add_stats(stats: &State<Arc<Stats>>, post: Json<PostData>, _key: ApiKey) -> Value {
    stats
        .guild_count
        .fetch_add(post.guild_count, Ordering::Relaxed);
    stats
        .total_pings
        .fetch_add(post.total_pings, Ordering::Relaxed);
    json!({"status": "ok"})
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Info stage", |rocket| async {
        rocket.mount("/api", routes![get_stats, add_stats])
    })
}
