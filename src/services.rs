use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{db, entry::{Entry, EntryInJSON}};

#[get("/entries")]
pub async fn get_entries(pool: web::Data<db::Pool>) -> impl Responder {
    let entries = db::execute(&pool, db::get_all_entries).await;

    if entries.is_err() {
        return HttpResponse::InternalServerError()
            .body("");
    }
    let entries = entries.unwrap();

    let body = serde_json::to_string(&entries.into_iter().map(|e| EntryInJSON::from(e)).collect::<Vec<_>>()).unwrap();

    log::info!("{}", body);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/entries/{entry_id}")]
pub async fn get_entries_with_id(pool: web::Data<db::Pool>, path: web::Path<u32>) -> impl Responder {
    let entry_id = path.into_inner();

    let entry = db::execute(&pool, db::get_entry_with_id(entry_id)).await;

    if entry.is_err() {
        return HttpResponse::NotFound()
            .body("");
    }
    let entry = entry.unwrap();

    let body = serde_json::to_string(&EntryInJSON::from(entry)).unwrap();

    log::info!("{}", body);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

fn get_iso_time() -> String {
    use chrono::prelude::*;
    
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true).to_string()
}

#[post("/entries")]
pub async fn post_entries(pool: web::Data<db::Pool>, req_body: String) -> impl Responder {
    let max_id = db::execute(&pool, db::get_max_id).await;

    if max_id.is_err() {
        return HttpResponse::InternalServerError()
            .body("");
    }
    let max_id = max_id.unwrap();

    let new_entry = Entry::from_post_entry(
        serde_json::from_str(&req_body).unwrap(),
        max_id + 1,
        get_iso_time()
    );

    let result = db::execute(&pool, db::post_new_entry(new_entry)).await;

    if result.is_err() {
        return HttpResponse::InternalServerError()
            .body("");
    }
    let new_entry = result.unwrap();

    let body = serde_json::to_string(&EntryInJSON::from(new_entry)).unwrap();

    log::info!("{}", body);

    HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
        .body("OK")
}
