use crate::{
    model::{track::TrackFile, trackpoint::TrackPoint},
    utils::{activity_type::sanitize_activity_type, file_utils::read_file},
};
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::{io::Error, path::Path};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Metadata {
    #[serde(rename = "name")]
    name: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct TrkPt {
    #[serde(rename = "@lat")]
    latitude: Option<f32>,
    #[serde(rename = "@lon")]
    longitude: Option<f32>,
    #[serde(rename = "ele")]
    elevation: Option<f32>,
    #[serde(rename = "time")]
    time: Option<String>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct TrkSeg {
    #[serde(rename = "trkpt")]
    points: Option<Vec<TrkPt>>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Track {
    #[serde(rename = "name")]
    name: Option<String>,
    #[serde(rename = "type")]
    activity_type: Option<String>,
    #[serde(rename = "trkseg")]
    segment: Option<TrkSeg>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Gpx {
    #[serde(rename = "@xmlns")]
    xmlns: Option<String>,
    #[serde(rename = "@xmlns:xsi")]
    xmlns_xsi: Option<String>,
    #[serde(rename = "@schemaLocation")]
    xsi_schema_location: Option<String>,
    #[serde(rename = "@version")]
    version: Option<String>,
    #[serde(rename = "@creator")]
    creator: Option<String>,
    #[serde(rename = "metadata")]
    metadata: Option<Metadata>,
    #[serde(rename = "trk")]
    track: Option<Track>,
}

fn get_activity_type(gpx: &Gpx) -> Result<String, Error> {
    if let Some(ref track) = gpx.track {
        if let Some(ref activity_type) = track.activity_type {
            let activity_type = sanitize_activity_type(activity_type);
            return Ok(activity_type);
        }
    }
    Ok("other".to_string())
}

fn get_track_points(gpx: &Gpx) -> Result<Vec<TrackPoint>, Error> {
    let mut track_points: Vec<TrackPoint> = Vec::new();
    if let Some(track) = &gpx.track {
        if let Some(segment) = &track.segment {
            if let Some(points) = &segment.points {
                for point in points {
                    let latitude = point.latitude;
                    let longitude = point.longitude;
                    let elevation = point.elevation;
                    let time = point.time.clone();

                    // If any point does not have the complete information the skip it
                    if latitude.is_none()
                        || longitude.is_none()
                        || elevation.is_none()
                        || time.is_none()
                    {
                        eprintln!("Point without complete information will be skipped");
                        continue;
                    }

                    let track_point = TrackPoint::new(
                        latitude.unwrap(),
                        longitude.unwrap(),
                        elevation.unwrap(),
                        time.unwrap(),
                    );
                    track_points.push(track_point);
                }
            }
        }
    }
    Ok(track_points)
}

pub fn read_gpx(path: &Path) -> Result<TrackFile, Error> {
    let raw_file = read_file(path)?;

    let gpx = from_str::<Gpx>(&raw_file).unwrap();
    let activity_type = get_activity_type(&gpx)?;
    let track_points = get_track_points(&gpx)?;

    Ok(TrackFile::new(track_points, activity_type))
}
