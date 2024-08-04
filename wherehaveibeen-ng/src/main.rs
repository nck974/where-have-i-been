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
use model::track::TrackInformation;
use utils::{
    cache_utils::save_cached_coordinates,
    environment::{get_cache_directory, get_tracks_directory},
    file_utils::{create_folder, get_valid_gps_files},
    gpx_utils::get_track_information,
};

fn initialize_data() {
    let mut conn = get_database_connection().unwrap();

    initialize_database(&mut conn).unwrap();

    // Get what is already stored in the database to avoid processing again the same files
    // that have already been processed
    let processed_files = read_files_in_database(&mut conn);

    // Create the folder where the simplified gpx tracks will be stored
    let cache_directory = get_cache_directory();
    let cache_path = Path::new(&cache_directory);
    create_folder(&cache_path).unwrap();

    let tracks_directory = get_tracks_directory();
    let path = Path::new(&tracks_directory);
    let files = get_valid_gps_files(path).unwrap();
    for filename in files {
        if processed_files.contains(&filename) {
            continue;
        }
        if let Ok((track_information, coordinates)) =
            get_track_information(path.join(&filename).as_path())
        {
            insert_file(&mut conn, &filename, track_information, false).unwrap();
            save_cached_coordinates(cache_path, &filename, coordinates).unwrap();
        } else {
            eprintln!("No track information found for {}", filename);
            insert_file(
                &mut conn,
                &filename,
                TrackInformation::new(0.0, 0.0, 0.0, 0.0),
                true,
            )
            .unwrap();
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
