use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub fn list_files_in_directory(path: &Path) -> Result<Vec<String>, Error> {
    let mut file_list = Vec::new();

    if !path.is_dir() {
        return Err(Error::new(
            ErrorKind::Other,
            "Provided path is not a directory.",
        ));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(filename) = entry.file_name().to_str() {
            if filename.ends_with(".gpx") {
                file_list.push(filename.to_string());
            }
        }
    }

    Ok(file_list)
}

pub fn read_file(path: &Path) -> Result<String, Error> {
    println!("Reading file: {}", path.display());
    fs::read_to_string(path)
}
