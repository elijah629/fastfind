const USAGE: &str = r"Super fast file finder

Usage: fastfind [GLOB_PATTERN]

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
    let glob_pattern = match args.get(1) {
        Some(x) => x,
        None => usage(),
    };

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        usage();
    }

    if args.contains(&"-V".to_string()) || args.contains(&"--version".to_string()) {
        println!("fastfind v0.1.1");
        return;
    }
    
    let current_dir = std::env::current_dir().unwrap();

    for entry in glob::glob_with(glob_pattern.as_str(), glob::MatchOptions { case_sensitive: false, require_literal_separator: false,
    require_literal_leading_dot: false }).expect("Invalid glob pattern") {
        match entry {
            Ok(path) => println!("{}", current_dir.join(path).display()),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}
