use super::trackpoint::TrackPoint;

#[derive(Debug)]

pub struct TrackInformation {
    pub north_west_latitude: f32,
    pub north_west_longitude: f32,
    pub south_east_latitude: f32,
    pub south_east_longitude: f32,
    pub date: String,
    pub activity_type: String,
}

impl TrackInformation {
    pub fn new(
        north_west_latitude: f32,
        north_west_longitude: f32,
        south_east_latitude: f32,
        south_east_longitude: f32,
        date: String,
        activity_type: String,
    ) -> Self {
        TrackInformation {
            north_west_latitude,
            north_west_longitude,
            south_east_latitude,
            south_east_longitude,
            date,
            activity_type,
        }
    }

    pub fn create_empty_track() -> Self {
        TrackInformation {
            north_west_latitude: 0.0,
            north_west_longitude: 0.0,
            south_east_latitude: 0.0,
            south_east_longitude: 0.0,
            date: "".to_string(),
            activity_type: "".to_string(),
        }
    }
}

pub struct TrackFile {
    pub track_points: Vec<TrackPoint>,
    pub activity_type: String,
}
impl TrackFile {
    pub fn new(track_points: Vec<TrackPoint>, activity_type: String) -> Self {
        TrackFile {
            track_points,
            activity_type,
        }
    }
}
