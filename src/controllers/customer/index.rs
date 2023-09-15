use crate::utils::{error::ApiError, http_body::Message};
use actix_web::web::Json;

pub async fn main() -> Result<Json<Message>, ApiError> {
    Ok(Json(Message { msg: "OK" }))
}
