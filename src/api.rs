use crate::structs::{Stats, DbConn};
use rocket::{State};
use std::error::Error;

#[get("/stats")]
pub fn get_stats(db_conn: State<DbConn>) -> Result<Option<Json<Stats>>, Box<dyn Error>> {
    let mut row = db_conn.lock()
        .expect("Database connection lock")
        .prepare("SELECT * FROM bot_stats")?
        .query([])?;

    if let Some(x) = row.next()? {
        return Ok(Some(Json(Stats {
            server_count: x.get(0)?,
            total_pings: x.get(1)?,
            ppm: x.get(2)?,
            total_mem: x.get(3)?
        })))
    };
    Ok(None)

}

#[post("/stats", format = "json")]
pub fn add_stats() -> JsonValue {
    // db.lock()
    //     .expect("db connection lock")
    //     .query_row("SELECT server_count FROM bot_stats", [], |row| { row.get(0) });
    json!({
        "status": "Success"
    })
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Info stage", |rocket| async {
        rocket.mount("/", routes![invite, discord, source])
    })
}