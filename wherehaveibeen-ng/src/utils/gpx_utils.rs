use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

use crate::model::{coordinate::Coordinate, track::TrackInformation};
use quick_xml::{events::Event, reader::Reader};

pub fn get_track_information(path: &Path) -> Result<(TrackInformation, Vec<Coordinate>)> {
    println!("Extracting track information of file: {}", path.display());
    let coordinates = read_gpx_file_coordinates(&path.to_path_buf())?;

    let mut north_west_longitude: f32 = std::f32::NAN;
    let mut north_west_latitude: f32 = std::f32::NAN;
    let mut south_east_longitude: f32 = std::f32::NAN;
    let mut south_east_latitude: f32 = std::f32::NAN;

    for coordinate in &coordinates {
        if north_west_longitude.is_nan() || coordinate.longitude > north_west_longitude {
            north_west_longitude = coordinate.longitude;
        }

        if north_west_latitude.is_nan() || coordinate.longitude > north_west_latitude {
            north_west_latitude = coordinate.latitude;
        }

        if south_east_longitude.is_nan() || coordinate.longitude < south_east_longitude {
            south_east_longitude = coordinate.longitude;
        }

        if south_east_latitude.is_nan() || coordinate.longitude < south_east_latitude {
            south_east_latitude = coordinate.latitude;
        }
    }

    if north_west_longitude.is_nan()
        || north_west_latitude.is_nan()
        || south_east_longitude.is_nan()
        || south_east_latitude.is_nan()
    {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "No coordiantes found the provided file",
        ));
    }

    Ok((
        TrackInformation {
            north_west_longitude,
            north_west_latitude,
            south_east_longitude,
            south_east_latitude,
        },
        coordinates,
    ))
}

/// .
/// This function just reads the xml trackpoints of the XML. This may be changed in the future
/// to also take care of Reding multiple tracks within the file
///
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
fn read_gpx_file_coordinates(path: &PathBuf) -> Result<Vec<Coordinate>> {
    let file = File::open(path).unwrap();
    let buff_reader = BufReader::new(file);

    let mut reader = Reader::from_reader(Box::new(buff_reader));
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut coordinates: Vec<Coordinate> = Vec::new();
    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        // NOTE: this is the generic case when we don't know about the input BufRead.
        // when the input is a &str or a &[u8], we don't actually need to use another
        // buffer, we could directly call `reader.read_event()`
        match reader.read_event_into(&mut buf) {
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Error at position {}: {:?}", reader.buffer_position(), e),
                ))
            }
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"trkpt" => {
                    let longitude: Option<f32> = e
                        .attributes()
                        .filter_map(|a| a.ok())
                        .find(|attr| attr.key == quick_xml::name::QName(b"lon"))
                        .and_then(|attr| String::from_utf8_lossy(&attr.value).parse().ok());
                    let latitude: Option<f32> = e
                        .attributes()
                        .filter_map(|a| a.ok())
                        .find(|attr| attr.key == quick_xml::name::QName(b"lat"))
                        .and_then(|attr| String::from_utf8_lossy(&attr.value).parse().ok());
                    if latitude.is_some() && longitude.is_some() {
                        coordinates.push(Coordinate::new(latitude.unwrap(), longitude.unwrap()))
                    }
                }
                _ => (),
            },
            _ => (), // ignore other xml events
        }
        buf.clear(); // clear memory
    }
    Ok(coordinates)
}
