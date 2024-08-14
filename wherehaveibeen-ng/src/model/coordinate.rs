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


// This is a support class used to built the heatmap as f32 should not be used as hash keys
#[derive(Serialize, Eq, PartialEq, Hash, Debug)]
pub struct StringifiedCoordinate {
    #[serde(rename(serialize = "a"))]
    pub latitude: String,
    #[serde(rename(serialize = "o"))]
    pub longitude: String,
}

impl StringifiedCoordinate {
    pub fn new(latitude: String, longitude: String) -> StringifiedCoordinate {
        StringifiedCoordinate {
            latitude,
            longitude,
        }
    }
}
