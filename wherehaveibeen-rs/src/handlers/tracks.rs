use axum::extract::Path;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path as FilePath;

use crate::database::tracks::TracksDatabase;
use crate::model::track::TrackInformation;
use crate::utils::api_response::json_not_found;
use crate::utils::api_response::json_ok;
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
            return json_ok(json!({ "fileList": files })).into_response();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return json_not_found("No tracks could be found").into_response();
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
    let activity_type = params
        .get("activityType")
        .and_then(|v| v.parse::<String>().ok())
        .unwrap_or_default();

    let track_information = TrackInformation::new(
        north_west_latitude,
        north_west_longitude,
        south_east_latitude,
        south_east_longitude,
        "".to_string(), // date is not implemented yet
        activity_type,
    );
    dbg!(&track_information);

    let tracks_db = TracksDatabase::new().unwrap();
    match tracks_db.get_tracks_inside_location(track_information) {
        Ok(files) => {
            return json_ok(json!({ "fileList": files })).into_response();
        }
        Err(e) => {
            println!("Error: {}", e);
            return json_not_found("No tracks could be found").into_response();
        }
    }
}

pub async fn get_activity_types() -> impl IntoResponse {
    let tracks_db = TracksDatabase::new().unwrap();
    match tracks_db.get_all_activity_types() {
        Ok(activity_types) => {
            return json_ok(json!({ "activityTypes": activity_types })).into_response();
        }
        Err(e) => {
            println!("Error: {}", e);
            return json_not_found("No tracks could be found").into_response();
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
                .body(file)
                .unwrap()
                .into_response();
        }
        Err(e) => {
            println!("Error: {}", e);
            return json_not_found("Thr provided track could be found").into_response();
        }
    }
}

pub async fn get_track_coordinates(Path(filename): Path<String>) -> impl IntoResponse {
    let cache_directory = get_cache_directory();
    let cache_path = FilePath::new(&cache_directory);
    match read_cached_coordinates(cache_path.join(&filename).as_path()) {
        Ok(coordinates) => {
            return json_ok(&coordinates).into_response();
        }
        Err(e) => {
            println!("Error: {}", e);
            return json_not_found("Thr provided track could be found").into_response();
        }
    }
}
