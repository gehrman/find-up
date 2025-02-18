use clap::Parser;
use clap::ArgAction;
use std::env::current_dir;
use std::process::exit;

/// Search up the directory tree, starting in the current directory, for a file
/// named TARGET. Prints the full path of the file if found.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// the name of the file to search for (optional, default .env)
    target: Option<String>,

    /// Set the verbosity output of the output, e.g. -vvvv for debug output
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    if args.verbose > 2 {
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

    if args.verbose > 0 {
        println!("{} not found", needle);
    }
    exit(1)
}
