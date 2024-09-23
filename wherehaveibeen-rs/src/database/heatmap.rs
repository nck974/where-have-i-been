use std::collections::HashMap;

use rusqlite::{named_params, params, params_from_iter, Connection, Result};

use crate::{
    model::{
        coordinate::StringifiedCoordinate, heatmap::HeatmapCoordinate, track::TrackInformation,
    },
    utils::environment::get_database_path,
};

use super::query::heatmap::{
    CREATE_HEATMAP_INDEX, CREATE_HEATMAP_TABLE, FILTER_HEATMAP_IN_LOCATION, GET_NR_HEATMAP_ROWS,
    INSERT_DATA_INTO_HEATMAP, INSERT_OR_UPDATE_DATA_INTO_HEATMAP,
};

pub struct HeatmapDatabase {
    pub conn: Connection,
}

impl HeatmapDatabase {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(get_database_path())?;
        Ok(Self { conn })
    }

    pub fn initialize_table(&self) -> Result<(), rusqlite::Error> {
        self.conn.execute(CREATE_HEATMAP_TABLE, ())?;

        Ok(())
    }

    fn is_heatmap_table_empty(&self) -> Result<bool, rusqlite::Error> {
        let count: i32 = self
            .conn
            .query_row(GET_NR_HEATMAP_ROWS, [], |row| row.get(0))?;

        Ok(count == 0)
    }

    pub fn update_heatmap(
        &mut self,
        heatmap: &mut HashMap<StringifiedCoordinate, i32>,
    ) -> Result<(), rusqlite::Error> {
        if self.is_heatmap_table_empty()? {
            self.insert_data_in_bulk(heatmap)?;
        } else {
            println!("The database is not empty. Updates will be slower...");
            self.insert_data_or_update(heatmap)?;
        }

        Ok(())
    }

    fn insert_data_in_bulk(
        &mut self,
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
        let mut tx = self.conn.transaction()?;
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
                        tx = self.conn.transaction()?;
                    }
                }

                // Start a new query
                query = String::from(INSERT_DATA_INTO_HEATMAP);
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

    fn insert_data_or_update(
        &mut self,
        heatmap: &mut HashMap<StringifiedCoordinate, i32>,
    ) -> Result<(), rusqlite::Error> {
        if heatmap.len() == 0 {
            return Ok(());
        }

        let transaction_size = 1000;
        let mut tx = self.conn.transaction()?;
        let mut counter = 0;
        for (coordinate, frequency) in heatmap.into_iter() {
            if (counter % transaction_size) == (transaction_size - 1) {
                tx.commit()?;
                tx = self.conn.transaction()?;
            }
            tx.execute(
                INSERT_OR_UPDATE_DATA_INTO_HEATMAP,
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

    pub fn create_table_indices(&self) -> Result<(), rusqlite::Error> {
        let result = self.conn.execute(CREATE_HEATMAP_INDEX, []);

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
        &self,
        track_information: TrackInformation,
    ) -> Result<Vec<HeatmapCoordinate>> {
        let mut stmt = self.conn.prepare(FILTER_HEATMAP_IN_LOCATION)?;

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
}
