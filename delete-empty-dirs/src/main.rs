use std::env;
use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let root_dir = get_root_dir()?;
    let dirs = get_dirs_recursively(&root_dir)?;

    println!("Found {} directories to check.", dirs.len());

    for dir in &dirs {
        if is_dir_empty(&dir)? {
            fs::remove_dir(&dir)?;
            println!("Deleted empty directory: {}", dir.display());
        }
    }

    Ok(())
}

fn get_root_dir() -> Result<PathBuf, Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(Error::new(ErrorKind::InvalidInput, "Usage: delete-empty-folders <folder>"));
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

fn is_dir_empty(dir: &Path) -> Result<bool, Error> {
    let mut entries = fs::read_dir(dir)?;
    Ok(entries.next().is_none()) // True if the directory is empty
}
