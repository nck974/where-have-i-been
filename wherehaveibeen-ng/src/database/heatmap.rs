use std::collections::HashMap;

use rusqlite::{named_params, params, params_from_iter, Connection, Result};

use crate::{
    model::{
        coordinate::StringifiedCoordinate, heatmap::HeatmapCoordinate, track::TrackInformation,
    },
    utils::environment::get_database_path,
};

/// Creates the sqlite database table if it does not already exist
///
/// # Errors
///
/// This function will return an error if the database can not be created.
pub fn initialize_heatmap_table(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS heatmap (
            latitude REAL NOT NULL,
            longitude REAL NOT NULL,
            frequency INTEGER NOT NULL,
            PRIMARY KEY (latitude, longitude)
        );
    )",
        (), // no params
    )?;

    Ok(())
}

fn is_heatmap_table_empty(conn: &Connection) -> Result<bool, rusqlite::Error> {
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM heatmap;", [], |row| row.get(0))?;
    if count > 0 {
        println!("The database is not empty. Updates will be slower...");
    }
    Ok(count == 0)
}

fn insert_data_in_empty_database(
    conn: &mut Connection,
    heatmap: &mut HashMap<StringifiedCoordinate, i32>,
) -> Result<(), rusqlite::Error> {
    if heatmap.len() == 0 {
        return Ok(());
    }

    let transaction_size = 100; // How many queries before a commit
    let chunk_size = 1000; // How many rows per query

    let mut query = String::new();
    let mut params = Vec::new();

    let mut counter = 0;
    let mut transaction_counter = 0;
    let mut tx = conn.transaction()?;
    println!("Saving heatmap into the database...");
    for (coordinate, frequency) in heatmap.into_iter() {
        // Start a new query if this is the first in the chunk
        if counter % chunk_size == 0 {
            if !query.is_empty() {
                query.pop(); // Remove the trailing comma
                tx.execute(&query, params_from_iter(params.iter()))?;
                params.clear();
                transaction_counter += 1;
                // Commit all buffered transactions
                if transaction_counter % transaction_size == 0 {
                    tx.commit()?;
                    tx = conn.transaction()?;
                }
            }

            // Start a new query
            query = String::from("INSERT INTO heatmap (frequency, latitude, longitude) VALUES ");
        }

        // Add placeholders to the query
        query.push_str("(?, ?, ?),");

        // Push values into the params vector
        params.push(frequency.to_string());
        params.push(coordinate.latitude.to_string());
        params.push(coordinate.longitude.to_string());

        counter += 1;
    }

    // Execute any remaining query if there are leftover rows
    if !query.is_empty() {
        query.pop(); // Remove trailing comma
        tx.execute(&query, params_from_iter(params.iter()))?;
    }
    tx.commit()?;

    Ok(())
}

fn update_data_in_database(
    conn: &mut Connection,
    heatmap: &mut HashMap<StringifiedCoordinate, i32>,
) -> Result<(), rusqlite::Error> {
    if heatmap.len() == 0 {
        return Ok(());
    }

    let transaction_size = 1000;
    let mut tx = conn.transaction()?;
    let mut counter = 0;
    for (coordinate, frequency) in heatmap.into_iter() {
        if (counter % transaction_size) == (transaction_size - 1) {
            tx.commit()?;
            tx = conn.transaction()?;
        }
        tx.execute(
            "INSERT INTO heatmap (latitude, longitude, frequency)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(latitude, longitude) 
         DO UPDATE SET frequency = frequency + excluded.frequency;",
            params![
                coordinate.latitude.to_string(),
                coordinate.longitude.to_string(),
                frequency.to_string()
            ],
        )?;
        counter += 1;
    }
    tx.commit()?;

    Ok(())
}

pub fn update_heatmap(
    conn: &mut Connection,
    heatmap: &mut HashMap<StringifiedCoordinate, i32>,
) -> Result<(), rusqlite::Error> {
    if is_heatmap_table_empty(conn)? {
        insert_data_in_empty_database(conn, heatmap)?;
    } else {
        update_data_in_database(conn, heatmap)?;
    }

    Ok(())
}

pub fn create_heatmap_index(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    let result = conn.execute(
        "CREATE INDEX idx_lat_long ON heatmap (latitude, longitude);",
        [],
    );

    match result {
        Ok(_) => {
            println!("Index for heatmap table created.");
        }
        Err(_) => {
            println!("Index for heatmap table already exits.")
        }
    }

    Ok(())
}

pub fn get_heatmap_inside_location(
    track_information: TrackInformation,
) -> Result<Vec<HeatmapCoordinate>> {
    let conn = Connection::open(get_database_path()).unwrap();

    let mut stmt = conn.prepare(
        "
SELECT 
	latitude, longitude, frequency
FROM 
	heatmap h  
WHERE 
	h.latitude < :north_west_latitude AND 
	h.latitude  > :south_east_latitude 
	AND h.longitude > :north_west_longitude AND h.longitude < :south_east_longitude;",
    )?;

    let row_content = stmt
        .query_map(
            named_params! {
                ":north_west_latitude": track_information.north_west_latitude,
                ":north_west_longitude": track_information.north_west_longitude,
                ":south_east_latitude": track_information.south_east_latitude,
                ":south_east_longitude": track_information.south_east_longitude,
            },
            |row| {
                Ok((
                    row.get::<_, f32>(0)?,
                    row.get::<_, f32>(1)?,
                    row.get::<_, i32>(2)?,
                ))
            },
        )
        .unwrap();

    let mut heatmap: Vec<HeatmapCoordinate> = Vec::new();
    for row in row_content {
        match row {
            Ok((latitude, longitude, frequency)) => {
                heatmap.push(HeatmapCoordinate::new(
                    latitude.to_string(),
                    longitude.to_string(),
                    frequency.to_string(),
                ));
            }
            Err(e) => {
                eprintln!("Error retrieving heatmap: {}", e);
            }
        }
    }

    Ok(heatmap)
}
