use rusqlite::{Connection, Result};
use std::fs;



pub fn get_connection() ->Result<Connection> {
    let db_path = "to.db";
    if !std::path::Path::new(db_path).exists() {
        fs::File::create(db_path).expect("Failed to create database file");
    }
    let conn = Connection::open(db_path)?;
    Ok(conn)
}
pub fn init_db() {
    let schema = include_str!("../schema.sql");
    let conn = get_connection().expect("DB init failed");
    conn.execute_batch(schema).expect("Failed to execute schema");
}