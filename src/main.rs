use std::path::Path;

use walkdir::WalkDir;

const USAGE: &str = r"Super fast file finder

Usage: fastfind [SEARCH_DIRECTORY] [SEARCH_STRING]

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

    let search = match args.get(2) {
        Some(x) => x.as_str(),
        None => usage(),
    };

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        usage();
    }

    if args.contains(&"-V".to_string()) || args.contains(&"--version".to_string()) {
        println!("fastfind v1.0.0");
        return;
    }
    
    let dir = Path::new(dir);

    if !dir.exists() {
        eprintln!("Path {dir:?} does not exist"); 
        return;
    }

    let mut x = 0;
    for entry in WalkDir::new(dir).same_file_system(true) {
        x += 1;
        if let Ok(entry) = entry {
            if entry.file_name().to_str().unwrap().contains(search) {
                println!("{}", entry.path().display());
            }
        }
    }

    println!("{x} files scanned");
}
