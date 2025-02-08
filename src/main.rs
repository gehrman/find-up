use clap::Parser;
use std::env::current_dir;
use std::process::exit;

/// Search up the directory tree for file named TARGET
#[derive(Parser, Debug)]
#[command(version, about, long_about)]
struct Args {
    /// Optional file name to search for
    target: Option<String>,

    /// Control the verbosity
    #[arg(short, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("Called with {:?}", args);
    }

    let needle = match &args.target {
        Some(name) => name,
        None => ".env",
    };

    for dir in current_dir().unwrap().ancestors() {
        let file = dir.join(needle);
        if file.exists() {
            println!("{}", file.display());
            exit(0)
        }
    }
    if args.verbose {
        println!("{} not found", needle);
    }
    exit(1)
}
