use std::env::current_dir;
use std::process::exit;


fn main() {
    let needle = ".env";  // TODO: make this a CLI flag

    for dir in current_dir().unwrap().ancestors() {
        let file = dir.join(needle);
        if file.exists() {
            println!("{}", file.display());
            exit(0)
        }
    }
    println!(".env file not found");  // TODO: put this behind -v flag
    exit(1)
}
