use rusqlite::{named_params, params, Connection, Result};

use crate::{model::track::TrackInformation, utils::environment::get_database_path};

use super::query::track::{
    CREATE_TRACKS_TABLE, CREATE_TRACK_COORDINATES_INDEX, CREATE_TRACK_FILENAME_INDEX,
    GET_ALL_ACTIVITY_TYPES, GET_ALL_TRACK_FILENAMES, GET_TRACKS_INSIDE_LOCATION, INSERT_TRACK,
};

pub struct TracksDatabase {
    pub conn: Connection,
}

impl TracksDatabase {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(get_database_path())?;
        Ok(Self { conn })
    }

    pub fn initialize_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute(CREATE_TRACKS_TABLE, ())?;

        Ok(())
    }

    pub fn get_all_filenames(&self) -> Vec<String> {
        let mut select_statement = self.conn.prepare(GET_ALL_TRACK_FILENAMES).unwrap();
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

    pub fn create_table_indices(&self) -> Result<(), rusqlite::Error> {
        let index_queries = vec![CREATE_TRACK_FILENAME_INDEX, CREATE_TRACK_COORDINATES_INDEX];

        for index_query in index_queries {
            let result = self.conn.execute(index_query, []);

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

    pub fn insert_new_file(
        &self,
        filename: &str,
        track_information: TrackInformation,
        is_empty_track: bool,
    ) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            INSERT_TRACK,
            params![
                filename,
                track_information.north_west_latitude,
                track_information.north_west_longitude,
                track_information.south_east_latitude,
                track_information.south_east_longitude,
                is_empty_track,
                track_information.date,
                track_information.activity_type
            ],
        )?;

        Ok(())
    }

    pub fn get_tracks_inside_location(
        &self,
        track_information: TrackInformation,
    ) -> Result<Vec<String>> {
        let mut query = String::from(GET_TRACKS_INSIDE_LOCATION);

        if !track_information.activity_type.is_empty() {
            query.push_str(" AND t.activity_type = :activity_type");
        } else {
            // TODO: Solve this in a cleaner way. named_params does not allow conditionally adding filters
            query.push_str(
                " AND (t.activity_type = :activity_type OR t.activity_type != :activity_type)",
            );
        }

        let mut stmt = self.conn.prepare(&query)?;

        let params = named_params! {
            ":provided_north_west_latitude": track_information.north_west_latitude,
            ":provided_north_west_longitude": track_information.north_west_longitude,
            ":provided_south_east_latitude": track_information.south_east_latitude,
            ":provided_south_east_longitude": track_information.south_east_longitude,
            ":activity_type": track_information.activity_type
        };

        let filenames = stmt
            .query_map(params, |row| Ok(row.get::<_, String>(0)?))
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

    pub fn get_all_activity_types(&self) -> Result<Vec<String>> {
        let query = String::from(GET_ALL_ACTIVITY_TYPES);

        let mut stmt = self.conn.prepare(&query)?;

        let activities = stmt
            .query_map((), |row| Ok(row.get::<_, String>(0)?))
            .unwrap();

        let mut activity_types = Vec::new();
        for activity in activities {
            match activity {
                Ok(f) => {
                    activity_types.push(f);
                }
                Err(e) => {
                    eprintln!("Error retrieving activity: {}", e);
                }
            }
        }
        dbg!(&activity_types);

        Ok(activity_types)
    }
}
