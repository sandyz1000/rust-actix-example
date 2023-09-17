#[cfg(test)]
mod tests {
    use crate::actors::PingerActor;
    use crate::handlers;
    use crate::messages::*;
    use actix::actors::mocker::Mocker;
    use actix::{Actor, Addr};
    use actix_web::body::Body;
    use actix_web::http::StatusCode;
    use actix_web::web;

    pub type PingerActorMock = Mocker<PingerActor>;

    #[actix_web::test]
    async fn test_deliver() {
        let mocker = PingerActorMock::mock(Box::new(move |_msg, _ctx| {
            let result: Result<Pong, std::io::Error> = Ok(Pong {});
            Box::new(Some(result))
        }));

        let actor: Addr<PingerActorMock> = mocker.start();

        let result = handlers::deliver::<PingerActorMock>(web::Data::new(actor)).await;

        assert!(result.is_ok());

        let http_result = result.unwrap();

        assert_eq!(http_result.status(), StatusCode::OK);

        let result_body = body_as_text(http_result.body());

        assert_eq!(result_body, "Pong");
    }

    fn body_as_text(body: &Body) -> String {
        match body {
            Body::Bytes(bytes) => {
                String::from_utf8(bytes.to_vec()).expect("Cannot convert bytes to string")
            }
            _ => panic!("Invalid result"),
        }
    }

    use actix_web::{get, http::header::ContentType, test, App, HttpResponse, Responder};

    #[get("/")]
    async fn index() -> impl Responder {
        HttpResponse::Ok().body("Hello world!")
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(
            App::new().service(index)
        ).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_post() {
        let app = test::init_service(
            App::new().service(index)
        ).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }
}
