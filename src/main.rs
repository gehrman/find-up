use std::env::current_dir;


fn main() {
    for dir in current_dir().unwrap().ancestors() {
        println!("Dir is {}", dir.display());
    }
}
