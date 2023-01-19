use std::env;
use std::fs;

fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    // Prints and returns the value of a given expression for quick and dirty debugging.
    // dbg!(args);

    if args.len() != 3 {
        return;
    }

    let _query = &args[1];
    let _file_path = &args[2];

    println!("Searching for {}", _query);
    println!("In file {}", _file_path);

    let contents = fs::read_to_string(_file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
