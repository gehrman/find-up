use clap::Parser;
use std::env::current_dir;
use std::process::exit;

/// Search up the directory tree, starting in the current directory, for a file
/// named TARGET.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// the name of the file to search for (optional, default .env)
    target: Option<String>,

    /// Use verbose output
    #[arg(short, long, default_value_t = false)]
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
