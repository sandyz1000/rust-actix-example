use actix_web::{web, App, HttpServer};

mod assets;
mod controllers;
mod db;
mod middlewares;
mod regex;
mod routes;
mod utils;

#[macro_use]
extern crate validator_derive;

#[derive(Debug)]
pub struct AppState {
    db_postgres: sqlx::PgPool,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    utils::cli::read_cli();

    let postgres_session = db::postgres::create_connection().await.unwrap();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                db_postgres: postgres_session.clone(),
            }))
            .configure(routes::routes)
    })
    .workers(num_cpus::get())
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
