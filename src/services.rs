use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{db, entry::{Entry, EntryInJSON}};

#[get("/entries")]
pub async fn get_entries(pool: web::Data<db::Pool>) -> impl Responder {
    let entries = db::execute(&pool, db::get_all_entries).await;
    
    if let Ok(entries) = entries {
        let body = serde_json::to_string(
            &entries.into_iter()
                .map(|e| EntryInJSON::from(e))
                .collect::<Vec<_>>()
            ).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    } else {
        HttpResponse::InternalServerError()
            .body("")
    }
}

#[get("/entries/{entry_id}")]
pub async fn get_entries_with_id(
    pool: web::Data<db::Pool>,
    path: web::Path<u32>
) -> impl Responder {
    let entry_id = path.into_inner();

    let entry = db::execute(&pool, db::get_entry_with_id(entry_id)).await;

    if let Ok(entry) = entry {
        let body = serde_json::to_string(&EntryInJSON::from(entry)).unwrap();

        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    } else {
        HttpResponse::NotFound()
            .body("Entry with that id was not found.")
    }
}

// A small litle function that returns the current time correctly formatted.
fn get_current_iso_time() -> String {
    use chrono::prelude::*;

    Utc::now()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
        .to_string()
}

#[post("/entries")]
pub async fn post_entries(
    pool: web::Data<db::Pool>,
    req_body: String
) -> impl Responder {
    // I didn't know how else to do this, but this works.
    let max_id = db::execute(&pool, db::get_max_id).await;

    if max_id.is_err() {
        return HttpResponse::InternalServerError()
            .body("");
    }
    let max_id = max_id.unwrap();

    let parsed_post = serde_json::from_str(&req_body);
    if parsed_post.is_err() {
        return HttpResponse::BadRequest()
            .body("Fields missing or not json.");
    }

    let new_entry = Entry::from_post_entry(
        parsed_post.unwrap(),
        max_id + 1,
        get_current_iso_time()
    );

    if new_entry.title.len() > 120 {
        return HttpResponse::BadRequest()
            .body("The length of the title can't exceed 120 characters.");
    }

    let result = db::execute(&pool, db::post_new_entry(new_entry)).await;

    if let Ok(new_entry) = result {
        let body = serde_json::to_string(&EntryInJSON::from(new_entry)).unwrap();

        log::info!("{}", body);

        HttpResponse::Created()
            .content_type("application/json")
            .body(body)
    } else {
        HttpResponse::InternalServerError()
            .body("")
    }
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
        .body("OK")
}
