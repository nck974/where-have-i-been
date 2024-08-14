use rusqlite::{named_params, params, Connection, Result};

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
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            latitude REAL NOT NULL,
            longitude REAL NOT NULL,
            frequency INTEGER NOT NULL
        );
    )",
        (), // no params
    )?;

    Ok(())
}

pub fn update_heatmap(
    conn: &Connection,
    coordinate: &StringifiedCoordinate,
    frequency: &i32,
) -> Result<(), rusqlite::Error> {
    let mut stmt =
        conn.prepare("SELECT frequency FROM heatmap WHERE latitude = ?1 AND longitude = ?2;")?;
    let result: Result<i32> = stmt
        .query_row(params![coordinate.latitude, coordinate.longitude], |row| {
            row.get(0)
        });

    match result {
        Ok(current_frequency) => {
            let new_frequency = current_frequency + frequency;
            conn.execute(
                "UPDATE heatmap SET frequency = ?1 WHERE latitude = ?2 AND longitude = ?3;",
                params![new_frequency, coordinate.latitude, coordinate.longitude],
            )?;
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            conn.execute(
                "INSERT INTO heatmap (frequency, latitude, longitude) VALUES (?1, ?2, ?3)",
                params![frequency, coordinate.latitude, coordinate.longitude],
            )?;
        }
        Err(e) => {
            eprintln!("An error occurred inserting data in the database: {}", e);
            return Err(e);
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
	h.latitude > :north_west_latitude AND 
	h.latitude  < :south_east_latitude 
	AND h.longitude < :north_west_longitude AND h.longitude > :south_east_longitude;",
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
