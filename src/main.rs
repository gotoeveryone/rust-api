use actix_web::{App, Error, HttpServer};
use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    api_v2_operation,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{self, Json},
    Apiv2Schema,
    OpenApiExt,
};
use serde::Serialize;

#[derive(Serialize, Apiv2Schema)]
struct State {
    name: String,
}

#[derive(Serialize, Apiv2Schema)]
struct User {
    id: u32,
    name: String,
}

#[api_v2_operation]
// #[get("/{id}/{name}")]
async fn user(path: web::Path<(u32, String)>) -> Result<Json<User>, Error> {
    let (id, name) = path.into_inner();
    Ok(web::Json(User { id: id, name: name }))
}

#[api_v2_operation]
// #[get("/")]
async fn index() -> Result<Json<State>, Error> {
    Ok(web::Json(State {
        name: "rust_api".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/{id}/{name}").route(web::get().to(user)))
            .with_json_spec_at("/spec")
            .build()
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
