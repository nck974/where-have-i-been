use serde::Serialize;

#[derive(Serialize)]
pub struct Coordinate {
    #[serde(rename(serialize = "a"))]
    pub latitude: f32,
    #[serde(rename(serialize = "o"))]
    pub longitude: f32,
}

impl Coordinate {
    pub fn new(latitude: f32, longitude: f32) -> Coordinate {
        Coordinate {
            latitude,
            longitude,
        }
    }
}
