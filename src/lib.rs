use eyre::eyre;
use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::iter::zip;
use std::path::{Path, PathBuf};
use strum_macros::Display;

/// Check if the path denoted by `directory` is actually an existing directory.
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

/// Check if the path denoted by `files` are actually an existing files.
/// It also checks that there is no duplicates in `files`.
fn check_files(files: &[impl AsRef<Path>]) -> eyre::Result<()> {
    // It's OK if `files` is empty.
    // Clap verifies that the list passed on the CLI is not empty so it's unlikely to get an empty list here.

    if files.len() == 1 && files[0].as_ref().is_dir() {
        return Err(eyre!(
            "A list of files is expected but you have passed a single directory"
        ));
    }

    for file in files {
        let file = file.as_ref();

        if !file.is_file() {
            return Err(eyre!(
                "'{}' does not exist or is a directory",
                file.display()
            ));
        }

        if file.extension() != Some("gpx".as_ref()) {
            return Err(eyre!(
                "'{}' does not appear to be a GPX file (since its extension is not '.gpx')",
                file.display()
            ));
        }
    }

    let unique = files
        .iter()
        .map(|as_ref_path| as_ref_path.as_ref().to_string_lossy().to_string())
        .collect::<HashSet<_>>();

    if unique.len() != files.len() {
        return Err(eyre!("There are duplicated files in the list"));
    }

    Ok(())
}

/// List the GPX files in a directory, based on the extensions.
fn list_gpx_files(directory: &impl AsRef<Path>) -> eyre::Result<Vec<PathBuf>> {
    assert!(directory.as_ref().is_dir());

    let mut gpx_files: Vec<PathBuf> = std::fs::read_dir(directory)
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
                    if path.extension().is_some_and(|ext| ext == "gpx") {
                        Some(path) // accept file
                    } else {
                        None // reject it
                    }
                }
                Err(e) => {
                    eprintln!("Cannot directory entry: {e}");
                    None
                }
            }
        })
        .collect();

    gpx_files.sort();

    Ok(gpx_files)
}

/// Load the GPX data from a file.
fn load_gpx(file: &impl AsRef<Path>) -> eyre::Result<gpx::Gpx> {
    assert!(file.as_ref().extension().is_some_and(|ext| ext == "gpx"));
    println!("Loading GPX from '{}'...", file.as_ref().display());

    let f = File::open(file)?;
    let reader = BufReader::new(f);
    let gpx = gpx::read(reader)?;
    Ok(gpx)
}

/// Save GPX content to a file.
fn save_gpx(gpx: &gpx::Gpx, file: &impl AsRef<Path>) -> eyre::Result<()> {
    assert!(file.as_ref().extension().is_some_and(|ext| ext == "gpx"));
    println!("Saving GPX to '{}'...", file.as_ref().display());

    let f = File::create(file)?;
    let writer = BufWriter::new(f);
    gpx::write(gpx, writer)?;
    Ok(())
}

#[derive(Display)]
enum Action {
    #[strum(serialize = "inverted")]
    Invert,
    #[strum(serialize = "merged")]
    Merge,
}

/// Construct of path of the output file for an operation on an input file or directory.
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
    if !value.is_empty() {
        println!("{key} = {value:?}");
    }
}

//----------------------------------------------------------------------------------------
// Functions for the commands

pub fn info(files: &[impl AsRef<Path>]) -> eyre::Result<()> {
    check_files(files)?;

    for file in files {
        let path = file.as_ref();

        println!("******************************************");
        println!("Info about {}", path.display());

        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let gpx = gpx::read(reader)?;

        // Version
        println!("GPX version = {}", gpx.version);
        print_option_field("Creator", &gpx.creator);

        println!("-- Metadata ------------------------------");

        if let Some(metadata) = gpx.metadata {
            print_option_field("Name", &metadata.name);
            print_option_field("Description", &metadata.description);
            print_option_field("Author", &metadata.author);
            print_vec_field("Links", &metadata.links);
            print_option_field("Time", &metadata.time);
            print_option_field("Keywords", &metadata.keywords);
            print_option_field("Copyright", &metadata.copyright);
            print_option_field("Bounds", &metadata.bounds);
        }

        println!("-- Waypoints -----------------------------");
        print_vec_field("Waypoints", &gpx.waypoints);

        println!("-- Tracks --------------------------------");
        for (i, item) in gpx.tracks.iter().enumerate() {
            println!("---- Track #{i}  ----------------------------");
            print_option_field("Name", &item.name);
            print_option_field("Comment", &item.comment);
            print_option_field("Description", &item.description);
            print_option_field("Source", &item.source);
            print_vec_field("Links", &item.links);
            print_option_field("Type", &item.type_);
            print_option_field("Number", &item.number);

            for (i, segment) in item.segments.iter().enumerate() {
                println!("Segment #{i} = {} points", segment.points.len())
            }
        }

        println!("-- Routes --------------------------------");
        print_vec_field("Routes", &gpx.routes);

        println!("******************************************");
    }

    Ok(())
}

fn get_creator() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

pub fn invert(files: &[impl AsRef<Path>]) -> eyre::Result<()> {
    check_files(files)?;

    let output_files = files
        .iter()
        .map(|f| get_output_file_path(f, Action::Invert))
        .collect::<Vec<_>>();

    for (in_file, out_file) in zip(files, output_files) {
        let mut content = load_gpx(&in_file)?;

        content.tracks.reverse();

        for track in &mut content.tracks {
            track.name = track.name.clone().map(|name| format!("{name} (inverted)"));
            track.segments.reverse();

            for segment in &mut track.segments {
                segment.points.reverse();
            }
        }

        content.creator = Some(get_creator());

        save_gpx(&content, &out_file)?;
    }

    Ok(())
}

pub fn invert_all(directory: &impl AsRef<Path>) -> eyre::Result<()> {
    check_directory(directory)?;
    let files = list_gpx_files(directory)?;

    if files.is_empty() {
        println!("No GPX files found in '{}'", directory.as_ref().display());
        return Ok(());
    }

    invert(&files)
}

pub fn merge(files: &[impl AsRef<Path>], output_file: &impl AsRef<Path>) -> eyre::Result<()> {
    check_files(files)?;

    println!("Merging {} files...", files.len());

    let contents = files.iter().map(load_gpx).collect::<Result<Vec<_>, _>>()?;

    let segments = contents
        .iter()
        .flat_map(|content| content.tracks.clone())
        .flat_map(|track| track.segments)
        .collect::<Vec<_>>();

    let track = gpx::Track {
        segments,
        ..Default::default()
    };

    let gpx = gpx::Gpx {
        version: gpx::GpxVersion::Gpx11,
        tracks: vec![track],
        ..Default::default()
    };

    save_gpx(&gpx, output_file)?;

    Ok(())
}

pub fn merge_all(directory: &impl AsRef<Path>) -> eyre::Result<()> {
    check_directory(directory)?;
    let files = list_gpx_files(directory)?;

    if files.is_empty() {
        println!("No GPX files found in '{}'", directory.as_ref().display());
        return Ok(());
    }

    let output_file = get_output_file_path(directory, Action::Merge);
    merge(&files, &output_file)
}
