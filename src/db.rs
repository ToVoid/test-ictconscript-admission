// use actix_web::{Error};

use actix_web::{error, web};
use rusqlite::Statement;

use crate::Entry;

pub type Pool = r2d2::Pool::<r2d2_sqlite::SqliteConnectionManager>;
type DBConnection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub async fn execute<T: Send + 'static>(
    pool: &Pool,
    query: impl FnOnce(DBConnection) -> rusqlite::Result<T> + Send + 'static
) -> Result<T, error::Error> {
    let pool = pool.clone();

    let connecection = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        query(connecection)
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

pub fn get_all_entries(connection: DBConnection) -> rusqlite::Result<Vec<Entry>> {
    let statement = connection.prepare(
        "SELECT * FROM entries
        ORDER BY isoTime;"
    )?;

    get_rows_as_entries(statement, [])
}

pub fn get_entry_with_id(id: u32) -> impl FnOnce(DBConnection) -> rusqlite::Result<Entry> {
    move |connection| {
        let statement = connection.prepare(
            "SELECT * FROM entries
            WHERE id is ?1"
        )?;

        let entries = get_rows_as_entries(statement, [id])?;

        if entries.len() == 1 {
            Ok(entries.into_iter().next().expect(""))
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
}

pub fn post_new_entry(new_entry: Entry) -> impl FnOnce(DBConnection) -> rusqlite::Result<Entry> {
    move |connection| {
        let mut statement = connection.prepare(
            "INSERT INTO entries (id, title, body, isoTime, lat, lon)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6);")?;

        statement.execute(rusqlite::params![
            new_entry.id,
            new_entry.title,
            new_entry.body,
            new_entry.iso_time,
            new_entry.lat,
            new_entry.lon
        ])?;

        Ok(new_entry)
    }
}

fn get_rows_as_entries<P: rusqlite::Params>(mut statement: Statement, params: P) -> rusqlite::Result<Vec<Entry>> {
    statement
        .query_map(params, |row| {
            Ok(
                Entry {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    body: row.get(2)?,
                    iso_time: row.get(3)?,
                    lat: row.get(4)?,
                    lon: row.get(5)?
                }
            )
        })
        .and_then(Iterator::collect)
}

pub fn get_max_id(connection: DBConnection) -> rusqlite::Result<u32> {
    connection.prepare(
        "SELECT MAX(id) FROM entries"
    )?.query_one([], |row| {
        row.get(0)
    })
}