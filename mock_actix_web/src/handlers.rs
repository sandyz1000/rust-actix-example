use actix::{Actor, Addr, Handler};
use actix::prelude::dev::ToEnvelope;
use actix_web::{Error, HttpResponse, web};
use actix_web::error::ErrorInternalServerError;

use crate::messages::*;

/// Routes incoming request to tenant router
pub async fn deliver<A>(actor: web::Data<Addr<A>>) -> Result<HttpResponse, Error>
    where A: Actor + Handler<Ping>,
          A::Context: ToEnvelope<A, Ping>, {
    let actor_request = actor.send(Ping {});

    let maybe_result = actor_request.await;

    maybe_result
        .map_err(|mailbox_err| ErrorInternalServerError(mailbox_err))?
        .map_err(|io_error| ErrorInternalServerError(io_error))
        .map(|_| HttpResponse::Ok().body("Pong"))
}
