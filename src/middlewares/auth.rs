// use actix_http::body::BoxBody;
use actix_service::{Service, Transform};
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    web::Data,
    Error, HttpMessage,
};
use futures::executor::block_on;
use futures::future::{ok, Ready};
use futures::Future;
use serde_json::Value as JsonValue;
use std::env::var;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::utils::{auth::decode_with_user_role, error::ApiError};

#[derive(Clone)]
pub enum AuthType {
    JWT(Vec<String>),
    APIKEY,
}

pub struct Auth {
    pub classification: AuthType,
}

impl<S> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service,
            classification: self.classification.clone(),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    classification: AuthType,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        match &self.classification {
            AuthType::JWT(role) => {
                let auth_cookie = req.cookie("Authorization");

                if auth_cookie == None {
                    return Box::pin(async {
                        let res =
                            req.error_response(ApiError::Unauthorized("not logged in".to_string()));
                        Ok(res)
                    });
                }

                let auth_cookie_unwrapped = auth_cookie.unwrap();

                let jwt_token = auth_cookie_unwrapped.value();

                let decoded = block_on(async {
                    decode_with_user_role(
                        role.clone(),
                        jwt_token,
                        &req.app_data::<Data<crate::AppState>>().unwrap(),
                    )
                    .await
                });

                if let Err(error) = decoded {
                    return Box::pin(async {
                        let res = req.error_response(error);
                        Ok(res)
                    });
                }

                req.extensions_mut().insert::<JsonValue>(
                    serde_json::from_str(&format!("{{ \"id\":{} }}", decoded.unwrap().id)).unwrap(),
                );
            }
            AuthType::APIKEY => {
                if let Some(key) = req.headers().get("x-api-key") {
                    if key.to_str().unwrap() != var("API_KEY").unwrap() {
                        return Box::pin(async {
                            let res = req
                                .error_response(ApiError::Unauthorized("wrong auth".to_string()));
                            Ok(res)
                        });
                    }
                } else {
                    return Box::pin(async {
                        Ok(req.error_response(ApiError::Unauthorized("wrong auth".to_string())))
                    });
                }
            }
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
