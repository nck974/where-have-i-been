use axum::Router;
mod routes;
mod handlers;
mod utils;
mod model;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/tracks", routes::tracks::router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
