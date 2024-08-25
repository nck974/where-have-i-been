use std::{
    f32::NAN,
    fs::File,
    io::{Error, ErrorKind},
    path::Path,
};

use fitparser::{from_reader, profile::MesgNum, FitDataField, FitDataRecord, Value};

use crate::model::{track::TrackFile, trackpoint::TrackPoint};

fn semicircles_to_degrees(semicircles: &i32) -> f32 {
    const SEMICIRCLES_TO_DEGREES: f32 = 180.0 / (2u32.pow(31) as f32);
    *semicircles as f32 * SEMICIRCLES_TO_DEGREES
}

fn get_coordinate_value(data_field: &FitDataField) -> Result<f32, Error> {
    if data_field.units() == "semicircles" {
        let value = data_field.value();
        match value {
            Value::SInt32(val) => {
                let value_degrees = semicircles_to_degrees(val);
                return Ok(value_degrees);
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unexpected value type")),
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "Unexpected units"))
}

fn get_elevation_value(data_field: &FitDataField) -> Result<f32, Error> {
    if data_field.units() == "m" {
        let value = data_field.value();
        match value {
            Value::Float64(val) => {
                return Ok(*val as f32);
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unexpected value type")),
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "Unexpected units"))
}

fn get_coordinate_timestamp(data_field: &FitDataField) -> Result<String, Error> {
    if data_field.units() == "s" {
        let value = data_field.value();
        match value {
            Value::Timestamp(val) => {
                let timestamp = val.to_rfc3339();
                return Ok(timestamp);
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unexpected value type")),
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "Unexpected units"))
}

fn get_activity_type(data_field: &FitDataField) -> Result<String, Error> {
    if data_field.units() == "" {
        let value = data_field.value();
        match value {
            Value::String(val) => {
                return Ok(val.clone());
            }
            _ => return Err(Error::new(ErrorKind::InvalidData, "Unexpected value type")),
        }
    }
    Err(Error::new(ErrorKind::InvalidData, "Unexpected units"))
}

fn get_record_trackpoint(record: FitDataRecord) -> Result<TrackPoint, Error> {
    let mut latitude: f32 = NAN;
    let mut longitude: f32 = NAN;
    let mut elevation: f32 = NAN;
    let mut time: String = String::new();

    for data_field in record.fields() {
        // println!("{:#?}", data_field);
        if data_field.name() == "position_lat" {
            latitude = get_coordinate_value(&data_field)?;
        } else if data_field.name() == "position_long" {
            longitude = get_coordinate_value(&data_field)?;
        } else if data_field.name() == "timestamp" {
            time = get_coordinate_timestamp(&data_field)?;
        } else if data_field.name() == "enhanced_altitude" {
            elevation = get_elevation_value(&data_field)?;
        }
    }

    // If any point does not have the complete information the skip it
    if latitude.is_nan() || longitude.is_nan() || time.is_empty() {
        eprintln!("Point without complete information will be skipped");
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Point without complete information",
        ));
    }

    Ok(TrackPoint::new(latitude, longitude, elevation, time))
}

fn get_session_activity_type(record: FitDataRecord) -> Result<String, Error> {
    for data_field in record.fields() {
        // println!("{:#?}", data_field);
        if data_field.name() == "sport" {
            return Ok(get_activity_type(&data_field)?);
        }
    }

    return Err(Error::new(
        ErrorKind::InvalidData,
        "Activity type not found",
    ));
}

fn get_track_file(data: Vec<FitDataRecord>) -> Result<TrackFile, Error> {
    let mut track_points: Vec<TrackPoint> = Vec::new();
    let mut activity_type: String = "other".to_string();
    for record in data {
        if record.kind() == MesgNum::Record {
            let trackpoint = get_record_trackpoint(record);
            match trackpoint {
                Err(_) => continue,
                Ok(point) => {
                    track_points.push(point);
                }
            }
        } else if record.kind() == MesgNum::Session {
            let activity_type_result = get_session_activity_type(record);
            match activity_type_result {
                Err(_) => continue,
                Ok(activity) => {
                    activity_type = activity;
                }
            }
        }
    }

    Ok(TrackFile::new(track_points, activity_type))
}

pub fn read_fit(path: &Path) -> Result<TrackFile, Error> {
    let mut fp = File::open(path)?;
    let data = from_reader(&mut fp);
    match data {
        Ok(vector) => return Ok(get_track_file(vector)?),
        Err(err) => {
            eprintln!("{:#?}", err);
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Activity type not found",
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_fit_file() {
        let file = Path::new("C:\\Users\\nck\\Development\\where-have-i-been\\wherehaveibeen-ng\\data\\track-fit\\1934901223.fit");
        let result = read_fit(&file);
        assert!(result.is_ok());
    }
}
