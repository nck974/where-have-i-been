#[derive(Debug)]

pub struct TrackInformation {
    pub north_west_latitude: f32,
    pub north_west_longitude: f32,
    pub south_east_latitude: f32,
    pub south_east_longitude: f32,
}

impl TrackInformation {
    pub fn new(
        north_west_latitude: f32,
        north_west_longitude: f32,
        south_east_latitude: f32,
        south_east_longitude: f32,
    ) -> Self {
        TrackInformation {
            north_west_latitude,
            north_west_longitude,
            south_east_latitude,
            south_east_longitude,
        }
    }
}
