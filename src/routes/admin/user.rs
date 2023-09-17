use crate::controllers::internal::user::handler::*;
use crate::middlewares::auth::{Auth, AuthType};
use actix_web::web::{get, scope, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .wrap(Auth {
                classification: AuthType::JWT(vec!["admin".to_string()]),
            })
            .route("/all", get().to(all)),
    );
}
