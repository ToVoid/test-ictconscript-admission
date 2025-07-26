use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{db, entry::EntryInJSON};

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
pub async fn get_entries_with_id(path: web::Path<u32>) -> impl Responder {
    let entry_id = path.into_inner();
    HttpResponse::NotImplemented()
}

#[post("/entries")]
pub async fn post_entries(_req_body: String) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok()
}
