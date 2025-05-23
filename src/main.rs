use clap::{Parser, Subcommand};
use merge_my_gpx::{decimate, info, invert, invert_all, merge, merge_all};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "MMG - A tool to merge GPX files")]
struct Cli {
    /// Enable verbose output.
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

const HELP_FOR_FILES_ARG: &str = "A list of path to your GPX files (separated with spaces).";
const HELP_FOR_DIRECTORY_ARG: &str = "The path of the directory where your GPX files are.";

#[derive(Subcommand)]
enum Command {
    /// Merge all tracks from all given files into a file with a single track.
    ///
    /// File are merged by order of appearance on the command-line.
    /// The output file `merged.gpx` is created in the current directory.
    Merge {
        #[arg(required = true, num_args = 1.., help = HELP_FOR_FILES_ARG)]
        files: Vec<PathBuf>,
    },

    /// Same as the "merge" command with all the files in the given directory.
    ///
    /// Files are merged by alphabetical order of their names.
    /// The output file `merged.gpx` is created in `directory`.
    #[command(name = "merge-all")]
    MergeAll {
        #[arg(required = true, help = HELP_FOR_DIRECTORY_ARG)]
        directory: PathBuf,
    },

    /// Invert each track of each given file.
    ///
    /// An output file is created per input file.
    /// Tracks and segments are not merged, just inverted one by one.
    Invert {
        #[arg(required = true, num_args = 1.., help = HELP_FOR_FILES_ARG)]
        files: Vec<PathBuf>,
    },

    /// Same as the "invert" command with all the files in the given directory.
    #[command(name = "invert-all")]
    InvertAll {
        #[arg(required = true, help = HELP_FOR_DIRECTORY_ARG)]
        directory: PathBuf,
    },

    /// Decimate the points of each (segment of each) track of each given file, to reduce their size.
    /// 
    /// For instance, Komoot cannot import a GPX file with too many points, and shows an error message like:
    /// 
    /// "There’s an issue with your file. It’s either too large or contains too many waypoints.
    /// Try importing multiple smaller files instead."
    /// 
    /// You can use this command to reduce the number of points until Komoot is happy.
    Decimate {
        #[arg(required = true, num_args = 1.., help = HELP_FOR_FILES_ARG)]
        files: Vec<PathBuf>,
        /// Decimate by a factor M; that is, keep only every M-th point.
        factor_m: u16,
    },

    /// Print information about one or more GPX files.
    Info {
        #[arg(required = true, num_args = 1.., help = HELP_FOR_FILES_ARG)]
        files: Vec<PathBuf>,
    },
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let execution_result = match &cli.command {
        Command::Invert { files } => invert(files),
        Command::InvertAll { directory } => invert_all(directory),
        Command::Merge { files } => merge(files, &std::env::current_dir()?.join("merged.gpx")),
        Command::MergeAll { directory } => merge_all(directory),
        Command::Info { files } => info(files),
        Command::Decimate { files, factor_m} => decimate(files, *factor_m),
    };

    match execution_result {
        Ok(_) => {
            println!("*** OK ***");
        }
        Err(err) => {
            if cli.verbose {
                eprintln!("*** Error: {:?}", err);
            } else {
                eprintln!("*** Error: {} ***", err);
            }
        }
    }

    Ok(())
}
