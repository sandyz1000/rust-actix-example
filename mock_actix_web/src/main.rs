use actix::prelude::*;
use actix_web::{App, HttpServer, web};

use crate::actors::PingerActor;

mod handlers;
mod actors;
mod messages;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(PingerActor::from_registry())
            .service(
                web::scope("/")
                    .route("ping", web::get().to(handlers::deliver::<PingerActor>))
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
