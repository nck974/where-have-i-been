#[derive(Debug)]

pub struct TrackPoint {
    pub latitude: f32,
    pub longitude: f32,
    pub elevation: f32,
    pub time: String,
}
impl TrackPoint {
    pub fn new(latitude: f32, longitude: f32, elevation: f32, time: String) -> Self {
        TrackPoint {
            latitude,
            longitude,
            elevation,
            time,
        }
    }
}
