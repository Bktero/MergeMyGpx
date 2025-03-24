use eyre::eyre;
use std::path::{Path, PathBuf};

fn check_directory(directory: &impl AsRef<Path>) -> eyre::Result<()> {
    let directory = directory.as_ref();

    if directory.is_dir() {
        Ok(())
    } else {
        Err(eyre!(
            "'{}' does not exist or is not a directory",
            directory.display()
        ))
    }
}

fn check_files(files: &[impl AsRef<Path>]) -> eyre::Result<()> {
    for file in files {
        let file = file.as_ref();

        if !file.is_file() {
            Err(eyre!(
                "'{}' does not exist or is a directory",
                file.display()
            ))?;
            // TODO: if len() == 1 and path is a directory, suggest another command
        }

        if file.extension() != Some("gpx".as_ref()) {
            Err(eyre!(
                "'{}' does not appear to be a GPX file (since its extension is not '.gpx')",
                file.display()
            ))?;
        }
    }

    Ok(())
}

fn list_gpx_files(directory: &impl AsRef<Path>) -> eyre::Result<Vec<PathBuf>> {
    debug_assert!(directory.as_ref().is_dir());

    let gpx_files: Vec<PathBuf> = std::fs::read_dir(directory)
        .map_err(|err| {
            eyre!(
                "Cannot read entries in directory '{}': {err}",
                directory.as_ref().display()
            )
        })?
        .filter_map(|res| {
            match res {
                Ok(dir_entry) => {
                    let path = dir_entry.path();
                    if path.extension().map_or(false, |ext| ext == "gpx") {
                        Some(path) // accept file
                    } else {
                        None // reject it
                    }
                }
                Err(e) => {
                    eprintln!("Error reading directory entry: {e}");
                    None
                }
            }
        })
        .collect();

    Ok(gpx_files)
}

pub fn info(file: &(impl AsRef<Path> + std::fmt::Debug)) -> eyre::Result<()> {
    check_files(&[file])?;
    Err(eyre!(
        "Not implemented yet: cannot give info about {:?}",
        file
    ))
}

pub fn invert(files: &[impl AsRef<Path> + std::fmt::Debug]) -> eyre::Result<()> {
    check_files(files)?;
    Err(eyre!(
        "Not implemented yet: cannot invert multiple files {:?}",
        files
    ))
}

pub fn invert_all(directory: &impl AsRef<Path>) -> eyre::Result<()> {
    check_directory(directory)?;
    let _files = list_gpx_files(directory)?;
    println!("{:#?}", _files);
    Err(eyre!(
        "Not implemented yet: cannot invert all in {:?} yet",
        directory.as_ref()
    ))
}

pub fn merge(files: &[impl AsRef<Path> + std::fmt::Debug]) -> eyre::Result<()> {
    check_files(files)?;
    Err(eyre!(
        "Not implemented yet: cannot merge multiple files {:?}",
        files
    ))
}

pub fn merge_all(directory: &impl AsRef<Path>) -> eyre::Result<()> {
    check_directory(directory)?;
    let _files = list_gpx_files(directory)?;
    println!("{:#?}", _files);
    Err(eyre!(
        "Not implemented yet: cannot merge all in {:?} yet",
        directory.as_ref()
    ))
}
