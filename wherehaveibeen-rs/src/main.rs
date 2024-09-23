mod database;
mod files;
mod handlers;
mod model;
mod routes;
mod utils;

use std::collections::HashMap;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

use axum::http::header::CONTENT_TYPE;
use axum::http::Method;
use axum::Router;
use database::heatmap::HeatmapDatabase;
use database::tracks::TracksDatabase;
use files::files::get_track_information;
use files::gz::decompress_all_gz_files;
use model::coordinate::{Coordinate, StringifiedCoordinate};
use model::track::TrackInformation;
use tower_http::cors::{Any, CorsLayer};
use utils::{
    cache_utils::save_cached_coordinates,
    environment::{get_cache_directory, get_tracks_directory},
    file_utils::{create_folder, get_valid_gps_files},
};

fn add_coordinates_to_heatmap(
    heatmap: &mut HashMap<StringifiedCoordinate, i32>,
    coordinates: &Vec<Coordinate>,
) {
    // First reduce the number of points that need to be inserted in the database by counting what
    // is already in memory
    for coordinate in coordinates {
        // Round the coordinate to minimize points (Lose approx 11m of precision), Usually it would
        // have 6 decimals but is now reduced to 5.
        let number_of_decimals: usize = 5;
        let rounded_coordinate = StringifiedCoordinate::new(
            format!("{:.1$}", coordinate.latitude, number_of_decimals),
            format!("{:.1$}", coordinate.longitude, number_of_decimals),
        );
        *heatmap.entry(rounded_coordinate).or_insert(0) += 1;
    }
}

fn initialize_data() {
    let start = Instant::now();

    let tracks_db = TracksDatabase::new().unwrap();
    let mut heatmap_db = HeatmapDatabase::new().unwrap();

    tracks_db.initialize_table().unwrap();
    heatmap_db.initialize_table().unwrap();

    // Get what is already stored in the database to avoid processing again the same files
    // that have already been processed
    let processed_files = tracks_db.get_all_filenames();

    // Create the folder where the simplified gpx tracks will be stored
    let cache_directory = get_cache_directory();
    let cache_path = Path::new(&cache_directory);
    create_folder(&cache_path).unwrap();

    let tracks_directory = get_tracks_directory();
    let path = Path::new(&tracks_directory);

    // Sometimes the .fit tracks are stored as .fit.gz
    decompress_all_gz_files(path).unwrap();

    let files = get_valid_gps_files(path).unwrap();
    let mut heatmap: HashMap<StringifiedCoordinate, i32> = HashMap::new();
    for filename in files {
        // Do not reprocess data already stored in the database for performance
        // on the second startup
        if processed_files.contains(&filename) {
            continue;
        }
        let file_path = path.join(&filename);

        let track = get_track_information(file_path.as_path());
        if let Ok((track_information, coordinates)) = track {
            tracks_db
                .insert_new_file(&filename, track_information, false)
                .unwrap();
            save_cached_coordinates(cache_path, &filename, &coordinates).unwrap();
            add_coordinates_to_heatmap(&mut heatmap, &coordinates);
        } else if let Err(e) = track {
            eprintln!("No track information found for {}", filename);
            eprintln!("Error: {}", e);
            // The file is still inserted to prevent duplicated analysis on the next restart
            tracks_db
                .insert_new_file(&filename, TrackInformation::create_empty_track(), true)
                .unwrap();
        }
    }

    println!("Saving heatmap...");
    if let Err(err) = heatmap_db.update_heatmap(&mut heatmap) {
        eprintln!("Error saving heatmap in the database: {}", err);
        exit(1)
    }

    println!("Creating indices...");
    heatmap_db.create_table_indices().unwrap();
    tracks_db.create_table_indices().unwrap();

    // Release connection
    tracks_db.conn.close().unwrap();
    heatmap_db.conn.close().unwrap();

    println!("Initialization took: {:?}", start.elapsed());
}

#[tokio::main]
async fn main() {
    println!("App is starting...");

    initialize_data();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .nest("/tracks", routes::tracks::router())
        .nest("/heatmap", routes::heatmap::router())
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
