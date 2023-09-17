use actix::prelude::*;
use std::io::Error;
use crate::messages::*;

#[derive(Debug)]
pub struct PingerActor {}

impl Default for PingerActor {
    fn default() -> Self {
        Self {}
    }
}

impl Actor for PingerActor {
    type Context = Context<Self>;
}

impl Supervised for PingerActor {
    fn restarting(&mut self, _: &mut Self::Context) {
        println!("Restarting...")
    }
}

impl SystemService for PingerActor {}

impl Handler<Ping> for PingerActor {
    type Result = Result<Pong, Error>;

    fn handle(&mut self, _: Ping, _: &mut Context<Self>) -> Self::Result {
        println!("Got ping!");
        Ok(Pong {})
    }
}
