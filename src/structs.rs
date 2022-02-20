use rusqlite::Connection;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

pub type DbConn = Mutex<Connection>;

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub server_count: u64,
    pub total_pings: u64,
    pub ppm: u64,
    pub total_mem: u64
}