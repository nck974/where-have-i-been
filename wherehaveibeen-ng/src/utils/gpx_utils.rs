use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Result},
    path::{Path, PathBuf},
};

use crate::model::coordinate::Coordinate;
use quick_xml::{events::Event, reader::Reader};

pub fn read_file_coordinates(path: &Path) -> Result<Vec<Coordinate>> {
    println!("Reading file: {}", path.display());
    read_gpx_file_coordinates(&path.to_path_buf())
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
