use actix_web::{error, get, App, Error, HttpRequest, HttpResponse, HttpServer};
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
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;

use entity::user;
use entity::user::Entity as User;

#[derive(Serialize, Apiv2Schema)]
struct State {
    name: String,
}

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

// #[api_v2_operation]
#[get("/users/{id}")]
async fn find_user(data: web::Data<AppState>, path: web::Path<i32>) -> Result<HttpResponse, Error> {
    let conn = &data.conn;
    let id = path.into_inner();
    let user = User::find_by_id(id)
        .one(conn)
        .await
        .expect("could not find user");

    match user {
        Some(value) => Ok(HttpResponse::Ok().json(web::Json(value))),
        None => Ok(HttpResponse::NotFound().body("not found")),
    }
}

#[api_v2_operation]
// #[get("/")]
async fn index() -> Result<Json<State>, Error> {
    Ok(web::Json(State {
        name: "rust_api".to_string(),
    }))
}

async fn not_found(_: web::Data<AppState>, __: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound().body("not found"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = sea_orm::Database::connect(&db_url).await.unwrap();
    let state = AppState { conn };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(find_user)
            .default_service(actix_web::web::route().to(not_found))
            .wrap_api()
            .service(web::resource("/").route(web::get().to(index)))
            // .service(web::resource("/{id}/").route(web::get().to(find_user)))
            .with_json_spec_at("/spec")
            .build()
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
