use std::io::{self, Read};

use actix_web::{web, error, App, HttpServer};
use r2d2_sqlite::SqliteConnectionManager;

mod db;
mod services;
mod entry;

use entry::Entry;

use crate::entry::EntryInJSON;

fn get_sample_data() -> io::Result<Vec<Entry>> {
    let mut file = std::fs::File::open("sample-data/data.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let data: Vec<EntryInJSON> = serde_json::from_str(&contents).unwrap();

    log::debug!("{:#?}", &data);

    Ok(data.into_iter().map(Entry::try_from).map(Result::unwrap).collect::<_>())
}

async fn setup_db(pool: &db::Pool) -> Result<(), error::Error> {
    let pool = pool.clone();

    // let connection = web::block(move ||)
    //     .await?
    //     .map_err(error::ErrorInternalServerError)?;

    web::block(move || {
        let connection = pool.get()
        .map_err(|_| rusqlite::Error::InvalidQuery)?;

        connection.execute(
            "CREATE TABLE entries (
                id INT NOT NULL,
                title VARCHAR(120) NOT NULL,
                body TEXT NOT NULL,
                isoTime VARCHAR(50),
                lat DECIMAL (7,4),
                lon DECIMAL (7,4),
                PRIMARY KEY (id)
            );",
            ()
        )?;

        let mut statement = connection.prepare(
            "INSERT INTO entries (id, title, body, isoTime, lat, lon)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6);"
        )?;
    
        for entry in get_sample_data().unwrap() {
            statement.execute(rusqlite::params![
                entry.id,
                entry.title,
                entry.body,
                entry.iso_time,
                entry.lat,
                entry.lon
            ])?;
        }
        
        Ok::<_, rusqlite::Error>(())
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let manager = SqliteConnectionManager::memory();
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();

    setup_db(&pool)
        .await
        .map_err(|err| { io::Error::other(err.to_string()) })?;

    log::info!("starting HTTP server at http://localhost:8080");
            
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(services::get_entries)
            .service(services::get_entries_with_id)
            .service(services::post_entries)
            .service(services::health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}