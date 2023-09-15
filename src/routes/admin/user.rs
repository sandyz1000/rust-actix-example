use actix_web::web::{get,ServiceConfig,scope};
use crate::controllers::internal::user::handler::*;
use crate::middlewares::auth::{Auth,AuthType};

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/user")
            .wrap(Auth{classification:AuthType::JWT(vec!["admin".to_string()])})
            .route("/all", get().to(all))
    );
}