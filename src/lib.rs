use eyre::eyre;
use std::collections::HashSet;
use std::fmt::{Debug, Display};
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

/// Load GPX data from a file.
fn load_gpx(file: &impl AsRef<Path>) -> eyre::Result<gpx::Gpx> {
    assert!(file.as_ref().extension().is_some_and(|ext| ext == "gpx"));
    println!("Loading GPX from '{}'...", file.as_ref().display());

    let f = File::open(file)?;
    let reader = BufReader::new(f);
    let gpx = gpx::read(reader)?;
    Ok(gpx)
}

/// Save GPX data to a file.
fn save_gpx(gpx: &gpx::Gpx, file: &impl AsRef<Path>) -> eyre::Result<()> {
    assert!(file.as_ref().extension().is_some_and(|ext| ext == "gpx"));
    println!("Saving GPX to '{}'...", file.as_ref().display());

    let f = File::create(file)?;
    let writer = BufWriter::new(f);
    gpx::write(gpx, writer)?;
    Ok(())
}

/// Get the value to use for the "creator" field for files we create.
fn get_creator() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[derive(Display)]
enum Action {
    #[strum(serialize = "decimated-by-{0}")]
    Decimate(u16),
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

fn print_field<T: Debug>(key: &str, value: T) {
    println!("{key} = {value:?}");
}

fn print_option_field_debug<T: Debug>(key: &str, option: &Option<T>) {
    if let Some(value) = option {
        println!("{key} = {value:?}");
    }
}

fn print_option_field<T: Display>(key: &str, option: &Option<T>) {
    if let Some(value) = option {
        println!("{key} = {value}");
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
            print_option_field_debug("Author", &metadata.author);
            print_vec_field("Links", &metadata.links);
            print_option_field_debug("Time", &metadata.time);
            print_option_field("Keywords", &metadata.keywords);
            print_option_field_debug("Copyright", &metadata.copyright);
            print_option_field_debug("Bounds", &metadata.bounds);
        }

        println!("-- Waypoints -----------------------------");
        for (i, waypoint) in gpx.waypoints.iter().enumerate() {
            println!("-- Waypoints #{i} --------------------------");
            print_option_field("Name", &waypoint.name);
            print_field("Point", &waypoint.point());
            print_option_field("elevation", &waypoint.elevation);
            print_option_field("comment", &waypoint.comment);
            print_option_field("description", &waypoint.description);
            print_option_field("source", &waypoint.source);
        }

        println!("-- Tracks --------------------------------");
        for (i, track) in gpx.tracks.iter().enumerate() {
            println!("---- Track #{i}  ----------------------------");
            print_option_field("Name", &track.name);
            print_option_field("Comment", &track.comment);
            print_option_field("Description", &track.description);
            print_option_field("Source", &track.source);
            print_vec_field("Links", &track.links);
            print_option_field("Type", &track.type_);
            print_option_field("Number", &track.number);

            for (i, segment) in track.segments.iter().enumerate() {
                println!("Segment #{i} = {} points", segment.points.len())
            }
        }

        let track_count = gpx.tracks.len();
        let segment_count = gpx
            .tracks
            .iter()
            .map(|track| track.segments.len())
            .sum::<usize>();
        let point_count = gpx
            .tracks
            .iter()
            .flat_map(|track| track.segments.clone())
            .map(|segment| segment.points.len())
            .sum::<usize>();

        println!(
            "Total: {} tracks / {} segments / {} points",
            track_count, segment_count, point_count
        );

        println!("-- Routes --------------------------------");
        print_vec_field("Routes", &gpx.routes);

        println!("******************************************");
    }

    Ok(())
}

pub fn invert(files: &[impl AsRef<Path>]) -> eyre::Result<()> {
    check_files(files)?;

    let output_files = files
        .iter()
        .map(|f| get_output_file_path(f, Action::Invert))
        .collect::<Vec<_>>();

    for (in_file, out_file) in zip(files, output_files) {
        let mut gpx = load_gpx(&in_file)?;

        gpx.tracks.reverse();

        for track in &mut gpx.tracks {
            track.name = track
                .name
                .clone()
                .map(|name| format!("{name} ({})", Action::Invert));
            track.segments.reverse();

            for segment in &mut track.segments {
                segment.points.reverse();
            }
        }

        gpx.creator = Some(get_creator());

        save_gpx(&gpx, &out_file)?;
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

    let gpxs = files.iter().map(load_gpx).collect::<Result<Vec<_>, _>>()?;

    let segments = gpxs
        .iter()
        .flat_map(|elem| elem.tracks.clone())
        .flat_map(|track| track.segments)
        .collect::<Vec<_>>();

    let track = gpx::Track {
        segments,
        ..Default::default()
    };

    let gpx = gpx::Gpx {
        creator: Some(get_creator()),
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

pub fn decimate(files: &[impl AsRef<Path>], factor_m: u16) -> eyre::Result<()> {
    check_files(files)?;

    let output_files = files
        .iter()
        .map(|f| get_output_file_path(f, Action::Decimate(factor_m)))
        .collect::<Vec<_>>();

    for (in_file, out_file) in zip(files, output_files) {
        let mut gpx = load_gpx(&in_file)?;

        for track in &mut gpx.tracks {
            track.name = track
                .name
                .clone()
                .map(|name| format!("{name} ({})", Action::Decimate(factor_m)));

            for segment in &mut track.segments {
                segment.points = segment
                    .points
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| i % factor_m as usize == 0 || *i == segment.points.len() - 1)
                    .map(|(_, element)| element.clone())
                    .collect::<Vec<_>>();
            }
        }

        gpx.creator = Some(get_creator());

        save_gpx(&gpx, &out_file)?;
    }

    Ok(())
}
