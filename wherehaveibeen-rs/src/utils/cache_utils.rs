use std::path::Path;

use crate::model::coordinate::Coordinate;

use super::file_utils::{read_file, save_to_file};

pub fn save_cached_coordinates(
    path: &Path,
    filename: &str,
    coordinates: &Vec<Coordinate>,
) -> Result<(), std::io::Error> {
    let mut content = String::new();
    for coordinate in coordinates {
        content.push_str(&format!(
            "{},{}\n",
            coordinate.latitude, coordinate.longitude
        ));
    }

    save_to_file(path.join(filename).as_path(), &content)
}

pub fn read_cached_coordinates(path: &Path) -> Result<Vec<Coordinate>, std::io::Error> {
    let content = read_file(path)?;

    let mut coordinates: Vec<Coordinate> = Vec::new();
    for line in content.lines() {
        let mut parts = line.split(',');
        if let (Some(lat_str), Some(lon_str)) = (parts.next(), parts.next()) {
            if let (Ok(latitude), Ok(longitude)) = (lat_str.parse(), lon_str.parse()) {
                coordinates.push(Coordinate::new(latitude, longitude));
            }
        }
    }

    Ok(coordinates)
}
