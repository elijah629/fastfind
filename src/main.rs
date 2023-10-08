use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{self, File},
    io::{prelude::*, BufReader, BufWriter, Write},
    path::PathBuf,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates an index for a directory and puts it in output
    Index {
        /// Directory to index
        directory: PathBuf,

        /// Output for the index.
        output: PathBuf,

        #[arg(short, long)]
        no_progress: bool,
    },

    /// Searches a sorted generated index.
    Search {
        /// Path for the generated index
        index: PathBuf,

        // Search string
        search: String,
    },
}

fn main() {
    let args = Args::parse();

    main_r(args).unwrap_or_else(|x| eprintln!("error: {}", x));
}

fn main_r(args: Args) -> Result<(), String> {
    match args.command {
        Commands::Index {
            directory,
            output,
            no_progress,
        } => {
            if !directory.exists() {
                return Err(format!("{directory:?} does not exist"));
            }

            if output.exists() {
                println!("warn: index file already exists, removing");
                fs::remove_file(&output).map_err(|_| "failed to remove index")?
            }

            let mut output =
                BufWriter::new(File::create(&output).map_err(|_| "failed to create index")?);

            let walker = WalkDir::new(directory)
                .same_file_system(true)
                .min_depth(1)
                .into_iter()
                .filter_map(|x| x.ok());

            if no_progress {
                for entry in walker {
                    let data = format!("{}\n", entry.path().display());

                    output
                        .write_all(data.as_bytes())
                        .map_err(|_| "failed to write to index")?;
                }
            } else {
                let bar = ProgressBar::new(0).with_style(
                    ProgressStyle::default_bar()
                        .template("{wide_bar} {pos}/{len} {eta}")
                        .unwrap(),
                );

                for entry in walker {
                    let data = format!("{}\n", entry.path().display());
                    output
                        .write_all(data.as_bytes())
                        .map_err(|_| "failed to write to index")?;
                    bar.inc(1);

                    let position = bar.position();
                    if bar.position() >= bar.length().unwrap() {
                        bar.set_length((position * 2) + (position / 2))
                    }
                }

                bar.finish();
            }
        }

        Commands::Search { index, search } => {
            let files = BufReader::new(File::open(index).map_err(|_| "failed to open index")?);

            for file in files.lines() {
                if let Ok(file) = file {
                    if file.contains(&search) {
                        println!("{}", file);
                    }
                }
            }
        }
    };

    Ok(())
}
