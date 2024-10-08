use std::env;

const DATABASE_PATH: &str = "tracks_database.db";
const CACHE_FOLDER: &str = ".//.cached_tracks";
const TRACKS_FOLDER: &str =
    "C:\\Users\\nck\\Development\\where-have-i-been\\wherehaveibeen-rs\\data\\track-fit-complete\\";
const ACTIVITY_TYPE_CONVERSIONS: &str = r###"{
        "StandUpPaddling": "Stand Up Paddling",
        "IceSkate": "Ice Skate",
        "generic": "other",
        "hiking": "walking",
        "ebikeride": "cycling",
        "Biking": "cycling",
        "biking": "cycling",
        "1": "cycling",
        "Velomobile": "cycling"
    }"###;

fn get_environment_variable(variable: &str, default: &str) -> String {
    env::var(&variable).unwrap_or(default.to_string())
}

pub fn get_database_path() -> String {
    get_environment_variable("DATABASE_PATH", DATABASE_PATH)
}

pub fn get_cache_directory() -> String {
    get_environment_variable("CACHE_DIRECTORY", CACHE_FOLDER)
}

pub fn get_tracks_directory() -> String {
    get_environment_variable("TRACKS_DIRECTORY", TRACKS_FOLDER)
}

pub fn get_activity_type_conversions() -> String {
    get_environment_variable("ACTIVITY_TYPE_CONVERSIONS", ACTIVITY_TYPE_CONVERSIONS)
}
