use std::env;
use std::process;

use minigrep::Config;

// Prints and returns the value of a given expression for quick and dirty debugging.
// dbg!(args);

//  cargo run > output.txt

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });




    println!("Searching for: \"{}\"", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
