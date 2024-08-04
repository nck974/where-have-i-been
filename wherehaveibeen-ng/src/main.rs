mod database;
mod handlers;
mod model;
mod routes;
mod utils;

use axum::Router;
use database::tracks_database::initialize_database;

#[tokio::main]
async fn main() {
    initialize_database();

    let app = Router::new().nest("/tracks", routes::tracks::router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
