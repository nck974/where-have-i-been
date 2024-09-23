use axum::extract::Query;
use axum::response::IntoResponse;
use std::collections::HashMap;

use crate::database::heatmap::HeatmapDatabase;
use crate::model::track::TrackInformation;
use crate::utils::api_response::json_not_found;
use crate::utils::api_response::json_ok;

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
        "".to_string(), // activity type is not available for heatmap
    );

    let heatmap_db = HeatmapDatabase::new().unwrap();
    match heatmap_db.get_heatmap_inside_location(track_information) {
        Ok(coordinates) => {
            return json_ok(&coordinates).into_response();
        }
        Err(e) => {
            println!("Error: {}", e);
            return json_not_found("No points where found in the given area").into_response();
        }
    }
}
