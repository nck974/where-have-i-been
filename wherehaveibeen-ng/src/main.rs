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

use axum::Router;
use database::heatmap::{create_heatmap_index, initialize_heatmap_table, update_heatmap};
use database::tracks::{
    create_tracks_index, get_database_connection, initialize_tracks_table, insert_file,
    read_files_in_database,
};
use files::files::get_track_information;
use model::coordinate::{Coordinate, StringifiedCoordinate};
use model::track::TrackInformation;
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

    let mut conn = get_database_connection().unwrap();

    initialize_tracks_table(&mut conn).unwrap();
    initialize_heatmap_table(&mut conn).unwrap();

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
            insert_file(&mut conn, &filename, track_information, false).unwrap();
            save_cached_coordinates(cache_path, &filename, &coordinates).unwrap();
            add_coordinates_to_heatmap(&mut heatmap, &coordinates);
        } else if let Err(e) = track {
            eprintln!("No track information found for {}", filename);
            eprintln!("Error: {}", e);
            insert_file(
                &mut conn,
                &filename,
                TrackInformation::create_empty_track(),
                true,
            )
            .unwrap();
        }
    }

    println!("Saving heatmap...");
    if let Err(err) = update_heatmap(&mut conn, &mut heatmap) {
        eprintln!("Error saving heatmap in the database: {}", err);
        exit(1)
    }

    println!("Creating indexes...");
    create_heatmap_index(&mut conn).unwrap();
    create_tracks_index(&mut conn).unwrap();

    // Release connection
    conn.close().unwrap();

    println!("Initialization took: {:?}", start.elapsed());
}

#[tokio::main]
async fn main() {
    initialize_data();

    let app = Router::new()
        .nest("/tracks", routes::tracks::router())
        .nest("/heatmap", routes::heatmap::router());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
