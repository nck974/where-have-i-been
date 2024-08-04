use std::fs;
use std::io;
use std::path::Path;

pub fn list_files_in_directory(path: &Path) -> io::Result<Vec<String>> {
    let mut file_list = Vec::new();

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".gpx") {
                    file_list.push(filename.to_string());
                }
            }
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Provided path is not a directory.",
        ));
    }

    Ok(file_list)
}

pub fn read_file(path: &Path) -> io::Result<String> {
    println!("Reading file: {}", path.display());
    fs::read_to_string(path)
}
