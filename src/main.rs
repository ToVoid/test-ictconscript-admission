use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/entries")]
async fn get_entries() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/entries/{entry_id}")]
async fn get_entries_with_id(_path: web::Path<u32>) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[post("/entries")]
async fn post_entries(_req_body: String) -> impl Responder {
    HttpResponse::NotImplemented()
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_entries)
            .service(get_entries_with_id)
            .service(post_entries)
            .service(health)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}