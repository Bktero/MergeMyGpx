use clap::{Parser, Subcommand};
use merge_my_gpx::{info, invert, invert_all, merge, merge_all};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "MMG - A tool to merge GPX files")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    //// Generate shell completion script
    // #[arg(long = "completion", value_enum)]
    // generate_completion: Option<Shell>,
    // ----
    //
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Merge all tracks from all files into a single file with a single track.
    ///
    /// File are merged by order of appearance on the command-line.
    Merge {
        #[arg(required = true, num_args = 1..)]
        files: Vec<PathBuf>,
        // TODO Add --smart option to try to guess the order of the tracks and the files when merging
    },
    /// Same as the "merge" command with all the files in the given directory.
    ///
    /// Files are merged by alphabetical order of their names.
    #[command(name = "merge-all")]
    MergeAll {
        #[arg(required = true)]
        directory: PathBuf,
    },

    /// Invert tracks in a file.
    /// Tracks are preserved.
    Invert {
        #[arg(required = true, num_args = 1..)]
        files: Vec<PathBuf>,
        // TODO add a --output-dir option
        // #[arg(short, long, required = false, help = "To override the default path for the ")]
        // output: Option<PathBuf>,
    },
    /// Same as the "invert" command with all the files in the given directory.
    #[command(name = "invert-all")]
    InvertAll {
        #[arg(required = true)]
        directory: PathBuf,
    },
    /// Print information about a GPX file.
    Info {
        #[arg(required = true)]
        file: PathBuf,
    },
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

    match &cli.command {
        Command::Invert { files } => {
            invert(files)?;
        }
        Command::InvertAll { directory } => {
            invert_all(directory)?;
        }
        Command::Merge { files } => {
            merge(files)?;
        }
        Command::MergeAll { directory } => {
            merge_all(directory)?;
        }
        Command::Info { file } => {
            info(file)?;
        }
    }

    Ok(())
}

// fn generate_completion<G: Generator>(cmd: &mut clap::Command, generator: G) {
//     generate(generator, cmd, cmd.get_name().to_string(), &mut io::stdout());
// }
