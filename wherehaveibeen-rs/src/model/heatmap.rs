use serde::Serialize;

#[derive(Debug, Serialize)]

pub struct HeatmapCoordinate {
    // Type is not relevant here as it will be just forwarded to the client
    #[serde(rename(serialize = "a"))]
    pub latitude: String,
    #[serde(rename(serialize = "o"))]
    pub longitude: String,
    #[serde(rename(serialize = "f"))]
    pub frequency: String,
}
impl HeatmapCoordinate {
    pub fn new(latitude: String, longitude: String, frequency: String) -> Self {
        HeatmapCoordinate {
            latitude,
            longitude,
            frequency,
        }
    }
}
