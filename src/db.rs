// use actix_web::{Error};

use actix_web::{error, web};
use rusqlite::Statement;

use crate::{Entry, EntryInJSON};

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

    get_rows_as_entries(statement)
}

fn get_rows_as_entries(mut statement: Statement) -> rusqlite::Result<Vec<Entry>> {
    statement
        .query_map([], |row| {
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