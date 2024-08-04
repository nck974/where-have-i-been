mod database;
mod handlers;
mod model;
mod routes;
mod utils;

use std::path::Path;

use axum::Router;
use database::tracks_database::{
    get_database_connection, initialize_database, insert_file, read_files_in_database,
};
use utils::{file_utils::list_files_in_directory, gpx_utils::get_track_information};

// TODO: Move this to a settings file
const BASE_PATH: &str =
    "C:\\Users\\nck\\Development\\where-have-i-been\\wherehaveibeen-ng\\data\\track-complete\\";

fn initialize_data() {
    let mut conn = get_database_connection().unwrap();

    initialize_database(&mut conn).unwrap();

    // Get what is already stored in the database to avoid processing again the same files
    // that have already been processed
    let processed_files = read_files_in_database(&mut conn);

    let path = Path::new(BASE_PATH);
    let files = list_files_in_directory(path).unwrap();
    for filename in files {
        if processed_files.contains(&filename) {
            continue;
        }
        if let Ok(track_information) = get_track_information(path.join(&filename).as_path()) {
            insert_file(&mut conn, &filename, track_information).unwrap();
        }else{
            eprintln!("No track information found for {}", filename);
        }
    }

    // Release connection
    conn.close().unwrap();
}

#[tokio::main]
async fn main() {
    initialize_data();

    let app = Router::new().nest("/tracks", routes::tracks::router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
