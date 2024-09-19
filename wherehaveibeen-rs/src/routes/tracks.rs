use axum::{
    routing::get,
    Router,
};
use crate::handlers::tracks::*;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_tracks))
        .route("/activity-types", get(get_activity_types))
        .route("/filtered-tracks", get(get_filtered_tracks))
        .route("/:filename", get(get_track))
        .route("/coordinates/:filename", get(get_track_coordinates))
}