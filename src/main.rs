use clap::{Parser, Subcommand};
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

        /// Seperates the filename from the directory in the index
        #[arg(short, long)]
        seperate_filename: bool,
    },

    /// Searches a sorted generated index.
    Search {
        /// Path for the generated index
        index: PathBuf,

        // Search string
        search: String,

        /// Does the index have the filename seperated? This will search the filename only if it is
        /// activated
        #[arg(short, long)]
        seperated: bool,
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
            seperate_filename,
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

            let mut completed = 0;

            for entry in WalkDir::new(directory)
                .same_file_system(true)
                .min_depth(1)
                .into_iter()
                .filter_map(|x| x.ok())
                .filter(|x| x.path().is_file())
            {
                let path = entry.path();
                if seperate_filename {
                    write!(
                        output,
                        "{}\t{}\n",
                        path.parent().unwrap().display(),
                        path.file_name().unwrap().to_str().unwrap()
                    )
                } else {
                    write!(output, "{}\n", entry.path().display())
                }
                .map_err(|_| "failed to write to index")
                .unwrap();

                completed += 1;
                print!("\rcompleted {completed}");
            }
            println!();
        }

        Commands::Search {
            index,
            search,
            seperated,
        } => {
            let files = BufReader::new(File::open(index).map_err(|_| "failed to open index")?);

            for file in files.lines().filter_map(|x| x.ok()) {
                if seperated {
                    let components = file.split_terminator('\t').collect::<Box<_>>();

                    if components[1].contains(&search) {
                        println!("{}/{}", components[0], components[1]);
                    }
                } else {
                    if file.contains(&search) {
                        println!("{}", file);
                    }
                }
            }
        }
    };

    Ok(())
}
