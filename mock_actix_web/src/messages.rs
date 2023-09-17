use actix::Message;
use std::io::Error;

pub struct Ping {}

impl Message for Ping {
    type Result = Result<Pong, Error>;
}

pub struct Pong {}
