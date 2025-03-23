use eyre::eyre;
use std::path::Path;

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
        }
    }

    Ok(())
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
    Err(eyre!(
        "Not implemented yet: cannot merge all in {:?} yet",
        directory.as_ref()
    ))
}
