use std::{
    io::{Error, ErrorKind},
    path::Path,
};

use chrono::DateTime;

use crate::model::{
    coordinate::Coordinate,
    track::{TrackFile, TrackInformation},
};

use super::{fit::read_fit, gpx::read_gpx};

fn extract_track_coordinates(track_file: &TrackFile) -> Vec<Coordinate> {
    let mut coordinates: Vec<Coordinate> = Vec::new();

    for point in &track_file.track_points {
        let coordinate = Coordinate::new(point.latitude, point.longitude);
        coordinates.push(coordinate)
    }

    coordinates
}
fn extract_track_information(track_file: &TrackFile) -> Result<TrackInformation, Error> {
    let mut north_west_longitude: f32 = std::f32::NAN;
    let mut north_west_latitude: f32 = std::f32::NAN;
    let mut south_east_longitude: f32 = std::f32::NAN;
    let mut south_east_latitude: f32 = std::f32::NAN;
    let mut date: String = "".to_string();
    let activity_type = track_file.activity_type.to_string();

    for coordinate in &track_file.track_points {
        if date.eq("") {
            date = coordinate.time.to_string();
        }

        if north_west_latitude.is_nan() || coordinate.latitude > north_west_latitude {
            north_west_latitude = coordinate.latitude;
        }

        if north_west_longitude.is_nan() || coordinate.longitude < north_west_longitude {
            north_west_longitude = coordinate.longitude;
        }

        if south_east_longitude.is_nan() || coordinate.longitude > south_east_longitude {
            south_east_longitude = coordinate.longitude;
        }

        if south_east_latitude.is_nan() || coordinate.latitude < south_east_latitude {
            south_east_latitude = coordinate.latitude;
        }
    }

    // Check all corners are valid points
    if north_west_longitude.is_nan()
        || north_west_latitude.is_nan()
        || south_east_longitude.is_nan()
        || south_east_latitude.is_nan()
    {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "No coordinates found the provided file",
        ));
    }

    // Check date format
    if let Ok(value) = DateTime::parse_from_rfc3339(&date) {
        date = value.to_rfc3339();
    } else {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid date format"));
    }

    Ok(TrackInformation::new(
        north_west_latitude,
        north_west_longitude,
        south_east_latitude,
        south_east_longitude,
        date,
        activity_type,
    ))
}

pub fn get_track_information(file: &Path) -> Result<(TrackInformation, Vec<Coordinate>), Error> {
    let track_file: TrackFile;
    if file.extension().unwrap() == "gpx" {
        track_file = read_gpx(file)?;
    } else if file.extension().unwrap() == "fit" {
        track_file = read_fit(file)?;
    } else {
        eprintln!("Invalid format {}", file.display());
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid format",
        ));
    }

    let coordinates = extract_track_coordinates(&track_file);
    let track_information = extract_track_information(&track_file)?;

    Ok((track_information, coordinates))
}
