use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Json;
use axum::response::Response;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path as FilePath;

use crate::database::tracks_database::get_tracks_inside_location;
use crate::model::track::TrackInformation;
use crate::utils::file_utils::list_files_in_directory;
use crate::utils::file_utils::read_file;
use crate::utils::gpx_utils::read_file_coordinates;

const BASE_PATH: &str =
    "C:\\Users\\nck\\Development\\where-have-i-been\\wherehaveibeen-ng\\data\\track-complete\\";

pub async fn get_tracks() -> impl IntoResponse {
    let path = FilePath::new(BASE_PATH);
    match list_files_in_directory(path) {
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
    let path = FilePath::new(BASE_PATH);
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
    let path = FilePath::new(BASE_PATH);
    match read_file_coordinates(path.join(&filename).as_path()) {
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
