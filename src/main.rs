use std::{path::Path, fs::{File, self}, io::{BufWriter, Write}};

use walkdir::WalkDir;

const USAGE: &str = r"Super fast file indexer

Usage: indexer [INDEX_DIRECTORY] [INDEX_OUTPUT]

Options:
  -h, --help        Print help
  -V, --version     Print version info and exit
";

#[inline]
fn usage() -> ! {
    eprintln!("{USAGE}");
    std::process::exit(1);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let dir = match args.get(1) {
        Some(x) => x,
        None => usage(),
    };

    let output = match args.get(2) {
        Some(x) => x,
        None => usage(),
    };

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        usage();
    }

    if args.contains(&"-V".to_string()) || args.contains(&"--version".to_string()) {
        println!("fastfind v{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    
    let dir = Path::new(dir);

    if !dir.exists() {
        eprintln!("error: {dir:?} does not exist"); 
        return;
    }

    if Path::new(output).exists() {
        println!("warn: index file already exists, removing");
        fs::remove_file(output).expect("error: failed to remove index");
    }

    let mut output = BufWriter::new(File::create(output).unwrap());
//    let cd = std::env::current_dir().unwrap();

    for entry in WalkDir::new(dir).same_file_system(true).min_depth(1).into_iter().filter_map(|x| x.ok()) {
            let data = format!("{}\n", entry.path().display());
            output.write_all(data.as_bytes()).expect("error: write error");
    }
}
