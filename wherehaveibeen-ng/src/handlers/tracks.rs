use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::response::Response;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path as FilePath;

use crate::database::tracks::get_tracks_inside_location;
use crate::model::track::TrackInformation;
use crate::utils::cache_utils::read_cached_coordinates;
use crate::utils::environment::get_cache_directory;
use crate::utils::environment::get_tracks_directory;
use crate::utils::file_utils::get_valid_gps_files;
use crate::utils::file_utils::read_file;

pub async fn get_tracks() -> impl IntoResponse {
    let tracks_directory = get_tracks_directory();
    let path = FilePath::new(&tracks_directory);
    match get_valid_gps_files(path) {
        Ok(files) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Json(json!({ "fileList": files })).into_response())
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(
                Json(json!({"message": "The provided track could not be found", "code": 404, "success": false}))
                    .to_string()
                    .into_response(),
            )
            .unwrap();
        }
    }
}
pub async fn get_filtered_tracks(
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
        "".to_string(), // dates are not implemented yet
    );
    dbg!(&track_information);

    match get_tracks_inside_location(track_information) {
        Ok(files) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Json(json!({ "fileList": files })).into_response())
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "application/json")
            .header("Access-Control-Allow-Origin", "*")
            .body(
                Json(json!({"message": "No tracks could be found", "code": 404, "success": false}))
                    .to_string()
                    .into_response(),
            )
            .unwrap();
        }
    }
}

pub async fn get_track(Path(filename): Path<String>) -> impl IntoResponse {
    let tracks_directory = get_tracks_directory();
    let path = FilePath::new(&tracks_directory);
    match read_file(path.join(&filename).as_path()) {
        Ok(file) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/gpx+xml")
                .header("Access-Control-Allow-Origin", "*")
                .body(file)
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(
                    Json(json!({"message": "The provided track could not be found", "code": 404, "success": false}))
                        .to_string()
                        .into(),
                )
                .unwrap();
        }
    }
}

pub async fn get_track_coordinates(Path(filename): Path<String>) -> impl IntoResponse {
    let cache_directory = get_cache_directory();
    let cache_path = FilePath::new(&cache_directory);
    match read_cached_coordinates(cache_path.join(&filename).as_path()) {
        Ok(coordinates) => {
            return Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(serde_json::to_string(&coordinates).unwrap())
                .unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(
                    Json(json!({"message": "The provided track could not be found", "code": 404, "success": false}))
                        .to_string()
                        .into(),
                )
                .unwrap();
        }
    }
}
