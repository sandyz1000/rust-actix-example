#[cfg(test)]
mod tests {
    use actix::actors::mocker::Mocker;
    use actix::{Addr, Actor};
    use actix_web::http::StatusCode;
    use crate::handlers;
    use crate::actors::PingerActor;
    use actix_web::web;
    use actix_web::body::Body;
    use crate::messages::*;

    pub type PingerActorMock = Mocker<PingerActor>;

    #[actix_rt::test]
    async fn test_deliver(){

        let mocker = PingerActorMock::mock(Box::new(move |_msg, _ctx| {
            let result: Result<Pong, std::io::Error> = Ok(Pong{});
            Box::new(Some(result))
        }));

        let actor: Addr<PingerActorMock> = mocker.start();

        let result = handlers::deliver::<PingerActorMock>(web::Data::new(actor)).await;

        assert!(result.is_ok());

        let http_result = result.unwrap();

        assert_eq!(http_result.status(), StatusCode::OK);

        let result_body = body_as_text(http_result.body().as_ref().unwrap());

        assert_eq!(result_body, "Pong");
    }

    fn body_as_text(body: &Body) -> String {
        match body {
            Body::Bytes(bytes) => {
                String::from_utf8(bytes.to_vec()).expect("Cannot convert bytes to string")
            },
            _ => panic!("Invalid result")
        }
    }

}
