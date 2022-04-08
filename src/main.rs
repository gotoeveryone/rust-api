use actix_web::{get, web, App, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct State {
    name: String,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[get("/{id}/{name}/")]
async fn user(path: web::Path<(u32, String)>) -> Result<impl Responder> {
    let (id, name) = path.into_inner();
    Ok(web::Json(User { id: id, name: name }))
}

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(web::Json(State {
        name: "rust_api".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(user))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
