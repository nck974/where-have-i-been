use std::{
    fs::{self, File},
    io::{Error, ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

use flate2::read::GzDecoder;

fn get_destination_path(src: &Path) -> Option<PathBuf> {
    let folder = src.parent().unwrap();
    let filename = src.file_stem().unwrap();
    return Some(folder.join(filename).to_path_buf());
}

fn decompress_file(src_path: &Path, destination: PathBuf) -> Result<(), Error> {
    println!("Decompressing {:#?}", src_path.display());

    let file = File::open(src_path)?;
    let mut decoder = GzDecoder::new(file);
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    let mut output_file = File::create(destination.as_path())?;
    output_file.write_all(&buffer)?;
    Ok(())
}

pub fn decompress_gz(src_path: &Path) -> Result<(), Error> {
    let dest_path = get_destination_path(src_path);

    match dest_path {
        Some(destination) => {
            // If the file has already been decompressed skip this step
            if destination.exists() {
                return Ok(());
            }

            decompress_file(src_path, destination)?;
        }
        None => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Destination path could not be created",
            ));
        }
    }
    Ok(())
}

pub fn decompress_all_gz_files(path: &Path) -> Result<(), Error> {
    if !path.is_dir() {
        return Err(Error::new(
            ErrorKind::Other,
            "Provided path is not a directory.",
        ));
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if let Some(filename) = entry.file_name().to_str() {
            if filename.ends_with(".gz") {
                let gz_file_path = path.join(filename);
                decompress_gz(&gz_file_path)?
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_gz_file() {
        let file = Path::new("C:\\Users\\nck\\Development\\where-have-i-been\\wherehaveibeen-rs\\data\\track-fit\\1934901223.fit.gz");
        let result = decompress_gz(&file);
        dbg!(&result);
        assert!(result.is_ok());
    }
    #[test]

    fn test_get_destination_path() {
        let file = Path::new("/Dev/track-fit/1934901223.fit.gz");
        let destination = get_destination_path(file).unwrap();
        let destination_path = destination.as_path();
        let expected = Path::new("/Dev/track-fit/1934901223.fit");
        assert_eq!(expected, destination_path);
    }
}
