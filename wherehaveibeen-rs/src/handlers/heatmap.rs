use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::response::Response;
use serde_json::json;
use std::collections::HashMap;

use crate::database::heatmap::get_heatmap_inside_location;
use crate::model::track::TrackInformation;

pub async fn get_filtered_heatmap(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let north_west_latitude = params
        .get("northWestLatitude")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or_default();
    let north_west_longitude = params
        .get("northWestLongitude")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or_default();
    let south_east_latitude = params
        .get("southEastLatitude")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or_default();
    let south_east_longitude = params
        .get("southEastLongitude")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or_default();

    let track_information = TrackInformation::new(
        north_west_latitude,
        north_west_longitude,
        south_east_latitude,
        south_east_longitude,
        "".to_string(), // date is not implemented yet
        "".to_string(), // activity type is not implemented yet
    );

    match get_heatmap_inside_location(track_information) {
        Ok(coordinates) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&coordinates).unwrap())
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "application/json")
            .body(
                Json(json!({"message": "The provided heatmap could not be found", "code": 404, "success": false}))
                        .to_string()
                        .into(),
            )
            .unwrap();
        }
    }
}
