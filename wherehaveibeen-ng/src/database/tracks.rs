use rusqlite::{named_params, params, Connection, Result};

use crate::{model::track::TrackInformation, utils::environment::get_database_path};

pub fn get_database_connection() -> Result<Connection> {
    Connection::open(get_database_path())
}

/// .Creates the sqlite database if it does not already exist
///
/// # Errors
///
/// This function will return an error if the database can not be created.
pub fn initialize_tracks_table(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            north_west_latitude REAL NOT NULL,
            north_west_longitude REAL NOT NULL,
            south_east_latitude REAL NOT NULL,
            south_east_longitude REAL NOT NULL,
            date DATE,
            is_empty_track INTEGER NOT NULL
        );
    )",
        (), // no params
    )?;

    Ok(())
}

pub fn read_files_in_database(conn: &mut Connection) -> Vec<String> {
    let mut select_statement = conn.prepare("SELECT filename FROM tracks;").unwrap();
    let rows = select_statement
        .query_map([], |row| Ok(row.get::<_, String>(0)?))
        .unwrap();
    let mut files: Vec<String> = Vec::new();
    for filename in rows {
        match filename {
            Ok(file) => files.push(file),
            Err(e) => {
                eprintln!("Error: Failed to read value - {}", e);
            }
        }
    }
    files
}

pub fn create_tracks_index(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    let index_queries = vec![
        "CREATE INDEX idx_filename ON tracks (filename);",
        "CREATE INDEX idx_square ON tracks (north_west_latitude, north_west_longitude, south_east_latitude, south_east_longitude);",
    ];

    for index_query in index_queries {
        let result = conn.execute(index_query, []);

        match result {
            Ok(_) => {
                println!("Index created.");
            }
            Err(_) => {
                println!("Index can not be created.")
            }
        }
    }

    Ok(())
}

pub fn insert_file(
    conn: &mut Connection,
    filename: &str,
    track_information: TrackInformation,
    is_empty_track: bool,
) -> Result<(), rusqlite::Error> {
    conn.execute(
        "INSERT INTO 
            tracks (
                filename, 
                north_west_latitude,
                north_west_longitude,
                south_east_latitude,
                south_east_longitude,
                is_empty_track,
                date
            ) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            filename,
            track_information.north_west_latitude,
            track_information.north_west_longitude,
            track_information.south_east_latitude,
            track_information.south_east_longitude,
            is_empty_track,
            track_information.date
        ],
    )?;

    Ok(())
}

pub fn get_tracks_inside_location(track_information: TrackInformation) -> Result<Vec<String>> {
    let conn = Connection::open(get_database_path()).unwrap();

    let mut stmt  = conn.prepare(
        "SELECT 
	filename
FROM 
	tracks t 
WHERE 
    is_empty_track IS FALSE
    AND
        (
            /* Check provided north west is contained inside one track limits*/
            (
                :provided_north_west_latitude<= t.north_west_latitude AND  :provided_north_west_latitude >= t.south_east_latitude AND
                :provided_north_west_longitude >= t.north_west_longitude AND :provided_north_west_longitude <= t.south_east_longitude
            )
            /* Check provided limits contain track north west*/
            OR (
                t.north_west_latitude <= :provided_north_west_latitude AND t.north_west_latitude >= :provided_south_east_latitude AND
                t.north_west_longitude >= :provided_north_west_longitude AND t.north_west_longitude  <= :provided_south_east_longitude
            )
            /* Check provided south west is contained inside one track limits*/
            OR (
                :provided_south_east_latitude<= t.north_west_latitude AND  :provided_south_east_latitude >= t.south_east_latitude AND
                :provided_north_west_longitude >= t.north_west_longitude AND :provided_north_west_longitude <= t.south_east_longitude
            )
            /* Check provided limits contain track south west*/
            OR (
                t.south_east_latitude <= :provided_north_west_latitude AND t.south_east_latitude >= :provided_south_east_latitude AND
                t.north_west_longitude >= :provided_north_west_longitude AND t.north_west_longitude  <= :provided_south_east_longitude
            )
            /* Check provided north east is contained inside one track limits*/
            OR(
                :provided_north_west_latitude<= t.north_west_latitude AND  :provided_north_west_latitude >= t.south_east_latitude AND
                :provided_south_east_longitude >= t.north_west_longitude AND :provided_south_east_longitude <= t.south_east_longitude
            )
            /* Check provided limits contain track north east*/
            OR (
                t.north_west_latitude <= :provided_north_west_latitude AND t.north_west_latitude >= :provided_south_east_latitude AND
                t.south_east_longitude >= :provided_north_west_longitude AND t.south_east_longitude  <= :provided_south_east_longitude
            )
            /* Check provided south east is contained inside one track limits*/
            OR (
                :provided_south_east_latitude<= t.north_west_latitude AND  :provided_south_east_latitude >= t.south_east_latitude AND
                :provided_south_east_longitude >= t.north_west_longitude AND :provided_south_east_longitude <= t.south_east_longitude
            )
            /* Check provided limits contain track south east*/
            OR (
                t.south_east_latitude <= :provided_north_west_latitude AND t.south_east_latitude >= :provided_south_east_latitude AND
                t.south_east_longitude >= :provided_north_west_longitude AND t.south_east_longitude  <= :provided_south_east_longitude
            )
        )
;"
    )?;

    let filenames = stmt
        .query_map(
            named_params! {
                ":provided_north_west_latitude": track_information.north_west_latitude,
                ":provided_north_west_longitude": track_information.north_west_longitude,
                ":provided_south_east_latitude": track_information.south_east_latitude,
                ":provided_south_east_longitude": track_information.south_east_longitude,
            },
            |row| Ok(row.get::<_, String>(0)?),
        )
        .unwrap();

    let mut files = Vec::new();
    for filename in filenames {
        match filename {
            Ok(f) => {
                files.push(f);
            }
            Err(e) => {
                eprintln!("Error retrieving filename: {}", e);
            }
        }
    }
    dbg!(&files);

    Ok(files)
}
