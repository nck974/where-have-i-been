pub const CREATE_HEATMAP_TABLE: &str = "
CREATE TABLE IF NOT EXISTS heatmap (
    latitude REAL NOT NULL,
    longitude REAL NOT NULL,
    frequency INTEGER NOT NULL,
    PRIMARY KEY (latitude, longitude)
);";

pub const GET_NR_HEATMAP_ROWS: &str = "SELECT COUNT(*) FROM heatmap;";

pub const INSERT_DATA_INTO_HEATMAP: &str = "
INSERT INTO
    heatmap (
        frequency, latitude, longitude) VALUES ";

pub const INSERT_OR_UPDATE_DATA_INTO_HEATMAP: &str = "
INSERT INTO heatmap (latitude, longitude, frequency)
VALUES (?1, ?2, ?3)
ON CONFLICT(latitude, longitude) 
DO UPDATE SET frequency = frequency + excluded.frequency;";

pub const CREATE_HEATMAP_INDEX: &str =
    "CREATE INDEX idx_lat_long ON heatmap (latitude, longitude);";

pub const FILTER_HEATMAP_IN_LOCATION: &str = "
SELECT 
	latitude, longitude, frequency
FROM 
	heatmap h  
WHERE 
	h.latitude < :north_west_latitude AND 
	h.latitude  > :south_east_latitude 
	AND h.longitude > :north_west_longitude AND h.longitude < :south_east_longitude;";
