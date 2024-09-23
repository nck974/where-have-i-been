pub const CREATE_TRACKS_TABLE: &str = "
    CREATE TABLE IF NOT EXISTS tracks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        filename TEXT NOT NULL,
        north_west_latitude REAL NOT NULL,
        north_west_longitude REAL NOT NULL,
        south_east_latitude REAL NOT NULL,
        south_east_longitude REAL NOT NULL,
        date DATE,
        is_empty_track INTEGER NOT NULL,
        activity_type TEXT NOT NULL
    );
)";

pub const GET_ALL_TRACK_FILENAMES: &str = "
    SELECT filename FROM tracks;
";

pub const CREATE_TRACK_FILENAME_INDEX: &str = "
    CREATE INDEX idx_filename ON tracks (filename);
";

pub const CREATE_TRACK_COORDINATES_INDEX: &str = "
CREATE INDEX 
    idx_square ON tracks (
        north_west_latitude,
        north_west_longitude,
        south_east_latitude,
        south_east_longitude
    );
";

pub const INSERT_TRACK: &str = "
INSERT INTO 
        tracks (
            filename, 
            north_west_latitude,
            north_west_longitude,
            south_east_latitude,
            south_east_longitude,
            is_empty_track,
            date,
            activity_type
        ) 
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
";

pub const GET_ALL_ACTIVITY_TYPES: &str = "
SELECT DISTINCT
    t.activity_type 
FROM tracks t 
WHERE
    t.activity_type != '' ORDER BY 1;";

pub const GET_TRACKS_INSIDE_LOCATION: &str = "SELECT 
	filename
FROM 
	tracks t 
WHERE 
    is_empty_track IS FALSE
    AND
(
    /* Check provided north west is contained inside one track limits*/
    (
        :provided_north_west_latitude<= t.north_west_latitude AND  :provided_north_west_latitude >= t.south_east_latitude AND
        :provided_north_west_longitude >= t.north_west_longitude AND :provided_north_west_longitude <= t.south_east_longitude
    )
    /* Check provided limits contain track north west*/
    OR (
        t.north_west_latitude <= :provided_north_west_latitude AND t.north_west_latitude >= :provided_south_east_latitude AND
        t.north_west_longitude >= :provided_north_west_longitude AND t.north_west_longitude  <= :provided_south_east_longitude
    )
    /* Check provided south west is contained inside one track limits*/
    OR (
        :provided_south_east_latitude<= t.north_west_latitude AND  :provided_south_east_latitude >= t.south_east_latitude AND
        :provided_north_west_longitude >= t.north_west_longitude AND :provided_north_west_longitude <= t.south_east_longitude
    )
    /* Check provided limits contain track south west*/
    OR (
        t.south_east_latitude <= :provided_north_west_latitude AND t.south_east_latitude >= :provided_south_east_latitude AND
        t.north_west_longitude >= :provided_north_west_longitude AND t.north_west_longitude  <= :provided_south_east_longitude
    )
    /* Check provided north east is contained inside one track limits*/
    OR(
        :provided_north_west_latitude<= t.north_west_latitude AND  :provided_north_west_latitude >= t.south_east_latitude AND
        :provided_south_east_longitude >= t.north_west_longitude AND :provided_south_east_longitude <= t.south_east_longitude
    )
    /* Check provided limits contain track north east*/
    OR (
        t.north_west_latitude <= :provided_north_west_latitude AND t.north_west_latitude >= :provided_south_east_latitude AND
        t.south_east_longitude >= :provided_north_west_longitude AND t.south_east_longitude  <= :provided_south_east_longitude
    )
    /* Check provided south east is contained inside one track limits*/
    OR (
        :provided_south_east_latitude<= t.north_west_latitude AND  :provided_south_east_latitude >= t.south_east_latitude AND
        :provided_south_east_longitude >= t.north_west_longitude AND :provided_south_east_longitude <= t.south_east_longitude
    )
    /* Check provided limits contain track south east*/
    OR (
        t.south_east_latitude <= :provided_north_west_latitude AND t.south_east_latitude >= :provided_south_east_latitude AND
        t.south_east_longitude >= :provided_north_west_longitude AND t.south_east_longitude  <= :provided_south_east_longitude
    )
)";