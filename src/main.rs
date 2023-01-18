use std::env;

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
}
