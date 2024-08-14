use axum::{
    routing::get,
    Router,
};
use crate::handlers::heatmap::*;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_filtered_heatmap))
}