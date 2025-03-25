use eyre::eyre;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use strum_macros::Display;

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
    // TODO what is files is empty?
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

    Ok(gpx_files) // TODO what is gpx_files is empty?
}

#[derive(Display)]
enum Action {
    #[strum(serialize = "inverted")]
    Invert,
    #[strum(serialize = "merged")]
    Merge,
}

fn get_output_file_path(path: &impl AsRef<Path>, action: Action) -> PathBuf {
    let path = path.as_ref();

    if path.is_dir() {
        path.join(action.to_string()).with_extension("gpx")
    } else {
        let ext = path.extension().expect("Path should have an extension");
        let stem = path
            .file_stem()
            .expect("Path should have a stem")
            .to_str()
            .expect("Should be able to convert to string")
            .to_owned()
            + "-"
            + &action.to_string();

        let base = path.parent().expect("Path should have a parent");
        base.join(stem).with_extension(ext)
    }
}

fn print_option_field<T: Debug>(key: &str, option: &Option<T>) {
    if let Some(value) = option {
        println!("{key} = {value:?}");
    }
}

fn print_vec_field<T: Debug>(key: &str, value: &Vec<T>) {
    if value.len() > 0 {
        println!("{key} = {value:?}");
    }
}

pub fn info(file: &(impl AsRef<Path> + Debug)) -> eyre::Result<()> {
    check_files(&[file])?;

    let path = file.as_ref();

    println!("------------------------------------------");
    println!("Info about {}", path.display());

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let gpx = gpx::read(reader)?;

    // Version
    println!("GPX version = {}", gpx.version);
    print_option_field("Creator", &gpx.creator);

    println!("-- Metadata ------------------------------");

    if let Some(metadata) = gpx.metadata {
        print_option_field("name", &metadata.name);
        print_option_field("description", &metadata.description);
        print_option_field("author", &metadata.author);
        print_vec_field("links", &metadata.links);
        // print_option_field("time", &metadata.time); // FIXME doesn't implement Debug
        print_option_field("keywords", &metadata.keywords);
        print_option_field("copyright", &metadata.copyright);
        // print_option_field("bounds", &metadata.bounds); // FIXME doesn't implement Debug
    }

    println!("-- Waypoints -----------------------------");
    print_vec_field("Waypoints", &gpx.waypoints);

    println!("-- Tracks --------------------------------");
    for (i, item) in gpx.tracks.iter().enumerate() {
        println!("-- Track #{i}  ------------------------------");
        print_option_field("Name", &item.name);
        print_option_field("comment", &item.comment);
        print_option_field("description", &item.description);
        print_option_field("source", &item.source);
        print_vec_field("links", &item.links);
        print_option_field("type", &item.type_);
        print_option_field("number", &item.number);
        println!("segments = {}", item.segments.len());
    }

    println!("-- Routes --------------------------------");
    print_vec_field("Routes", &gpx.routes);

    println!("------------------------------------------");
    Ok(())
}

pub fn invert(files: &[impl AsRef<Path> + Debug]) -> eyre::Result<()> {
    check_files(files)?;

    let output_files = files
        .iter()
        .map(|f| get_output_file_path(f, Action::Invert))
        .collect::<Vec<_>>();

    println!("{:#?}", files);
    println!("{:#?}", output_files);

    Err(eyre!("Not implemented yet: cannot invert multiple files"))
}

pub fn invert_all(directory: &impl AsRef<Path>) -> eyre::Result<()> {
    check_directory(directory)?;

    let input_files = list_gpx_files(directory)?;
    let output_file = get_output_file_path(directory, Action::Invert);

    println!("{:#?}", input_files);
    println!("{:#?}", output_file);

    Err(eyre!("Not implemented yet: cannot invert all"))
}

pub fn merge(files: &[impl AsRef<Path> + Debug]) -> eyre::Result<()> {
    check_files(files)?;

    let output_files = files
        .iter()
        .map(|f| get_output_file_path(f, Action::Merge))
        .collect::<Vec<_>>();

    println!("{:#?}", files);
    println!("{:#?}", output_files);

    Err(eyre!("Not implemented yet: cannot merge multiple files"))
}

pub fn merge_all(directory: &impl AsRef<Path>) -> eyre::Result<()> {
    check_directory(directory)?;

    let input_files = list_gpx_files(directory)?;
    let output_file = get_output_file_path(directory, Action::Merge);

    println!("{:#?}", input_files);
    println!("{:#?}", output_file);

    Err(eyre!("Not implemented yet: cannot merge all"))
}
