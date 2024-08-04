use rusqlite::{named_params, params, Connection, Result};

use crate::model::track::TrackInformation;

const DATABASE_NAME: &str = "tracks_database.db";

pub fn get_database_connection() -> Result<Connection> {
    Connection::open(DATABASE_NAME)
}

/// .Creates the sqlite database if it does not already exist
///
/// # Errors
///
/// This function will return an error if the database can not be created.
pub fn initialize_database(conn: &mut Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            filename TEXT NOT NULL,
            northwestlongitude REAL NOT NULL,
            northwestlatitude REAL NOT NULL,
            southeastlongitude REAL NOT NULL,
            southeastlatitude REAL NOT NULL,
            date DATE
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

pub fn insert_file(
    conn: &mut Connection,
    filename: &str,
    track_information: TrackInformation,
) -> Result<(), rusqlite::Error> {
    dbg!(&track_information);
    conn.execute(
        "INSERT INTO 
            tracks (
                filename, 
                northwestlongitude,
                northwestlatitude,
                southeastlongitude,
                southeastlatitude
            ) 
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            filename,
            track_information.north_west_longitude,
            track_information.north_west_latitude,
            track_information.south_east_longitude,
            track_information.south_east_latitude,
        ],
    )?;

    Ok(())
}

pub fn get_tracks_inside_location(track_information: TrackInformation) -> Result<Vec<String>> {
    let conn = Connection::open(DATABASE_NAME).unwrap();

    let mut stmt  = conn.prepare(
        "SELECT 
	filename
FROM 
	tracks t 
WHERE 
	/* Check provided north west is conained inside one track limits*/
	(
		:providednorthwestlatitude<= t.northwestlatitude AND  :providednorthwestlatitude >= t.southeastlatitude AND
		:providednorthwestlongitude >= t.northwestlongitude AND :providednorthwestlongitude <= t.southeastlongitude
	)
	/* Check provided limits contain track nort west*/
	OR (
		t.northwestlatitude <= :providednorthwestlatitude AND t.northwestlatitude >= :providedsoutheastlatitude AND
		t.northwestlongitude >= :providednorthwestlongitude AND t.northwestlongitude  <= :providedsoutheastlongitude
	)
	/* Check provided south west is conained inside one track limits*/
	OR (
		:providedsouthlatitude<= t.northwestlatitude AND  :providedsouthlatitude >= t.southeastlatitude AND
		:providednorthwestlongitude >= t.northwestlongitude AND :providednorthwestlongitude <= t.southeastlongitude
	)
	/* Check provided limits contain track south west*/
	OR (
		t.southeastlatitude <= :providednorthwestlatitude AND t.southeastlatitude >= :providedsoutheastlatitude AND
		t.northwestlongitude >= :providednorthwestlongitude AND t.northwestlongitude  <= :providedsoutheastlongitude
	)
	/* Check provided north east is conained inside one track limits*/
	OR(
		:providednorthwestlatitude<= t.northwestlatitude AND  :providednorthwestlatitude >= t.southeastlatitude AND
		:providedsoutheastlongitude >= t.northwestlongitude AND :providedsoutheastlongitude <= t.southeastlongitude
	)
	/* Check provided limits contain track nort east*/
	OR (
		t.northwestlatitude <= :providednorthwestlatitude AND t.northwestlatitude >= :providedsoutheastlatitude AND
		t.southeastlongitude >= :providednorthwestlongitude AND t.southeastlongitude  <= :providedsoutheastlongitude
	)
	/* Check provided south east is conained inside one track limits*/
	OR (
		:providedsouthlatitude<= t.northwestlatitude AND  :providedsouthlatitude >= t.southeastlatitude AND
		:providedsoutheastlongitude >= t.northwestlongitude AND :providedsoutheastlongitude <= t.southeastlongitude
	)
	/* Check provided limits contain track south east*/
	OR (
		t.southeastlatitude <= :providednorthwestlatitude AND t.southeastlatitude >= :providedsoutheastlatitude AND
		t.southeastlongitude >= :providednorthwestlongitude AND t.southeastlongitude  <= :providedsoutheastlongitude
	)
;"
    )?;

    let filenames = stmt
        .query_map(
            named_params! {
                ":providednorthwestlatitude": track_information.north_west_latitude,
                ":providednorthwestlongitude": track_information.north_west_longitude,
                ":providedsoutheastlatitude": track_information.south_east_latitude,
                ":providedsoutheastlongitude": track_information.south_east_longitude,
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
