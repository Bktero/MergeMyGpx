use clap::{Parser, Subcommand};
use merge_my_gpx::{info, invert, invert_all, merge, merge_all};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "MMG - A tool to merge GPX files")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    // TODO add completion
    //// Generate shell completion script
    // #[arg(long = "completion", value_enum)]
    // generate_completion: Option<Shell>,
    // ----
    //
    #[command(subcommand)]
    command: Command,
}

// TODO Add --smart option to "merge" and "merge-all" to try to guess the order of the tracks and the files when merging

const HELP_FOR_FILES_ARG: &str = "A list of path to your GPX files (separated with spaces).";
const HELP_FOR_DIRECTORY_ARG: &str = "The path of the directory where your GPX files are.";

#[derive(Subcommand)]
enum Command {
    /// Merge all tracks from all given files into a file with a single track.
    ///
    /// File are merged by order of appearance on the command-line.
    ///
    /// A unique output file is created. // FIXME where
    Merge {
        #[arg(required = true, num_args = 1.., help = HELP_FOR_FILES_ARG)]
        files: Vec<PathBuf>,
    },

    /// Same as the "merge" command with all the files in the given directory.
    ///
    /// Files are merged by alphabetical order of their names.
    #[command(name = "merge-all")]
    MergeAll {
        #[arg(required = true, help = HELP_FOR_DIRECTORY_ARG)]
        directory: PathBuf,
    },

    /// Invert each track of each given file.
    ///
    /// An output file is created per input file.
    /// Tracks are not merged, just inverted one by one.
    Invert {
        #[arg(required = true, num_args = 1.., help = HELP_FOR_FILES_ARG)]
        files: Vec<PathBuf>,
        // TODO add a --output-dir option
        // #[arg(short, long, required = false, help = "To override the default path for the ")]
        // output: Option<PathBuf>,
    },

    /// Same as the "invert" command with all the files in the given directory.
    #[command(name = "invert-all")]
    InvertAll {
        #[arg(required = true, help = HELP_FOR_DIRECTORY_ARG)]
        directory: PathBuf,
    },

    /// Print information about a GPX file.
    Info {
        /// The path of your GPX file.
        #[arg(required = true)]
        file: PathBuf,
    },
}

fn execute(cli: &Cli) -> eyre::Result<()> {
    match &cli.command {
        Command::Invert { files } => {
            invert(files)
        }
        Command::InvertAll { directory } => {
            invert_all(directory)
        }
        Command::Merge { files } => {
            merge(files)
        }
        Command::MergeAll { directory } => {
            merge_all(directory)
        }
        Command::Info { file } => {
            info(file)
        }
    }
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    if cli.verbose {
        println!("Verbose mode enabled");
        // TODO: Configure logging level based on verbosity
    }

    //// Check if completion generation was requested
    // if let Some(generator) = cli.generate_completion {
    //     let mut cmd = Cli::command();
    //     eprintln!("Generating completion file for {:?}...", generator);
    //     generate_completion(&mut cmd, generator);
    //     return Ok(());
    // }

    if let Err(err) = execute(&cli) {
        println!("Error: {:?}", err); // TODO use {} instead when software is more stable, or add a --debug (undocumented?) or --verbose option
    }

    Ok(())
}

// fn generate_completion<G: Generator>(cmd: &mut clap::Command, generator: G) {
//     generate(generator, cmd, cmd.get_name().to_string(), &mut io::stdout());
// }
