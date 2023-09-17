use super::error::ApiError;
use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Deserialize, Serialize)]
pub struct MessageWithData<T> {
    pub msg: &'static str,
    pub data: T,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub msg: &'static str,
}

pub fn get_data_from_middleware(req: &HttpRequest) -> Result<JsonValue, ApiError> {
    // TODO: Verify this Fix
    if let Some(id) = req.headers().get("id") {
        let json_value = serde_json::from_str(&format!("{{ \"id\":{:?} }}", id)).unwrap();
        return Ok(json_value);
    }
    Err(ApiError::NotFound("Header not found".to_string()))
}
