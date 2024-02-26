use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let root_dir = get_root_dir()?;
    let dirs = get_dirs_recursively(&root_dir)?;

    println!("Found {} directories to check.", dirs.len());

    for dir in dirs {
        if dir_contains_only_images(&dir)? {
            fs::remove_dir_all(&dir)?;
            println!("Deleted directory containing only images: {}", dir.display());
        }
    }

    Ok(())
}

fn get_root_dir() -> Result<PathBuf, Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "Usage: image-folder-cleanup <folder>"));
    }

    let path = PathBuf::from(&args[1]);
    if !path.is_dir() {
        return Err(Error::new(ErrorKind::NotFound, "Provided path is not a directory"));
    }

    Ok(path)
}

fn get_dirs_recursively(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut subdirs = Vec::new();
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            subdirs.push(path.clone());
            subdirs.extend(get_dirs_recursively(&path)?);
        }
    }

    Ok(subdirs)
}

fn dir_contains_only_images(dir: &Path) -> Result<bool, Error> {
    let mut file_count = 0;
    let mut image_count = 0;
    let entries = fs::read_dir(dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            return Ok(false); // Contains a subdirectory, so it's not a match
        } else if path.is_file() {
            file_count += 1;
            match path.extension().and_then(std::ffi::OsStr::to_str) {
                Some("jpg") | Some("jpeg") | Some("png") | Some("gif") => image_count += 1,
                _ => return Ok(false), // Contains a non-image file
            }
        }
    }

    Ok(file_count > 0 && file_count == image_count) // True if all files are images and at least one file exists
}
