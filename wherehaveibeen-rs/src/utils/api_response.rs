use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub fn json_ok<T>(body: T) -> impl IntoResponse
where
    T: serde::Serialize,
{
    (StatusCode::OK, Json(body))
}

pub fn json_not_found(message: &str) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(json!({ "message": message })))
}

// A helper function for a generic error response
// pub fn json_error(status: StatusCode, message: &str) -> impl IntoResponse {
//     (status, Json(json!({ "message": message })))
// }
