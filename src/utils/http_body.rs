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
    let headers = req.head().headers();
    if let Some(header_value) = headers.get("your_header_name") {
        if let Ok(header_value_str) = header_value.to_str() {
            if let Ok(json_value) = serde_json::from_str(header_value_str) {
                return Ok(json_value);
            }
        }
    }
    Err(ApiError::NotFound("Header not found".to_string()))
}
